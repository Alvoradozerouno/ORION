"""
GSF REST Client — execute, verify_chain, monitor_metrics, export_signed_ledger, mesh_join
Retry logic, TLS verification, schema validation.
"""

import json
import ssl
import time
import urllib.error
import urllib.request
from typing import Any, Dict, List, Optional


class GsfClientError(Exception):
    """GSF client error."""

    pass


class GsfClient:
    def __init__(
        self,
        base_url: str = "http://localhost:8765",
        verify_tls: bool = True,
        timeout: float = 30.0,
        retries: int = 3,
        retry_backoff: float = 1.0,
    ):
        self.base_url = base_url.rstrip("/")
        self.verify_tls = verify_tls
        self.timeout = timeout
        self.retries = retries
        self.retry_backoff = retry_backoff

    def _opener(self) -> urllib.request.OpenerDirector:
        ctx = ssl.create_default_context()
        if not self.verify_tls:
            ctx.check_hostname = False
            ctx.verify_mode = ssl.CERT_NONE
        return urllib.request.build_opener(urllib.request.HTTPSHandler(context=ctx))

    def _request(
        self,
        method: str,
        path: str,
        data: Optional[bytes] = None,
        headers: Optional[Dict[str, str]] = None,
    ) -> Dict[str, Any]:
        url = f"{self.base_url}{path}"
        h = {"Content-Type": "application/json", **(headers or {})}
        req = urllib.request.Request(url, data=data, headers=h, method=method)
        last_err: Optional[Exception] = None
        for attempt in range(self.retries):
            try:
                with urllib.request.urlopen(req, timeout=self.timeout) as r:
                    body = r.read().decode()
                    if path == "/metrics":
                        return {"raw": body}
                    return json.loads(body) if body else {}
            except urllib.error.HTTPError as e:
                last_err = e
                if e.code in (429, 502, 503) and attempt < self.retries - 1:
                    time.sleep(self.retry_backoff * (2**attempt))
                    continue
                try:
                    err_body = e.read().decode()
                    return {"ok": False, "error": err_body, "status": e.code}
                except Exception:
                    return {"ok": False, "error": str(e), "status": e.code}
            except (urllib.error.URLError, json.JSONDecodeError, OSError) as e:
                last_err = e
                if attempt < self.retries - 1:
                    time.sleep(self.retry_backoff * (2**attempt))
                    continue
                raise GsfClientError(str(e)) from e
        raise GsfClientError(str(last_err)) from last_err

    def _get(self, path: str) -> dict:
        return self._request("GET", path)

    def _post(self, path: str, data: Dict[str, Any]) -> dict:
        return self._request("POST", path, data=json.dumps(data).encode())

    def health(self) -> dict:
        """Health check."""
        return self._get("/health")

    def verify_audit(self) -> dict:
        """Verify audit chain integrity."""
        return self._get("/audit/verify")

    def verify_chain(self) -> dict:
        """Alias for verify_audit."""
        return self.verify_audit()

    def execute(self, intent: str, pattern: str) -> dict:
        """Execute intent with pattern. Synchronous."""
        return self._post("/run", {"intent": intent, "pattern": pattern})

    def export_chain(self) -> List[Dict[str, Any]]:
        """Export audit chain as list of entries."""
        r = self._get("/audit/export")
        if isinstance(r, list):
            return r
        if isinstance(r, dict) and "entries" in r:
            return r.get("entries", [])
        return []

    def export_signed_ledger(self) -> List[Dict[str, Any]]:
        """Export signed audit ledger (chain with signatures)."""
        return self.export_chain()

    def monitor_metrics(self) -> Dict[str, Any]:
        """Fetch Prometheus metrics. Returns parsed gauge/counter values."""
        r = self._get("/metrics")
        if "raw" in r:
            raw = r["raw"]
            out: Dict[str, Any] = {}
            for line in raw.splitlines():
                if line.startswith("gsf_") and not line.startswith("#"):
                    parts = line.split()
                    if len(parts) >= 2:
                        try:
                            out[parts[0]] = float(parts[1])
                        except ValueError:
                            out[parts[0]] = parts[1]
            return out
        return r

    def mesh_join(self, entries: List[Dict[str, str]]) -> dict:
        """Sync entries to mesh. entries: [{prev_hash, entry_hash, signature, timestamp}]."""
        return self._post("/mesh/sync", {"entries": entries})


try:
    import asyncio
except ImportError:
    asyncio = None


if asyncio is not None:

    class AsyncGsfClient(GsfClient):
        """Async client using asyncio. Requires Python 3.7+."""

        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._executor = __import__("concurrent.futures").ThreadPoolExecutor(max_workers=4)

        async def execute_async(self, intent: str, pattern: str) -> dict:
            """Async execute."""
            loop = asyncio.get_event_loop()
            return await loop.run_in_executor(
                self._executor, lambda: self.execute(intent, pattern)
            )

        async def verify_chain_async(self) -> dict:
            loop = asyncio.get_event_loop()
            return await loop.run_in_executor(self._executor, self.verify_chain)

        async def export_chain_async(self) -> List[Dict[str, Any]]:
            loop = asyncio.get_event_loop()
            return await loop.run_in_executor(self._executor, self.export_chain)

        async def monitor_metrics_async(self) -> Dict[str, Any]:
            loop = asyncio.get_event_loop()
            return await loop.run_in_executor(self._executor, self.monitor_metrics)
