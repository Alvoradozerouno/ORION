"""
GSF REST Client — execute, verify_audit, export_chain
"""

import urllib.request
import json
from typing import Optional


class GsfClient:
    def __init__(self, base_url: str = "http://localhost:8765"):
        self.base_url = base_url.rstrip("/")

    def _get(self, path: str) -> dict:
        req = urllib.request.Request(f"{self.base_url}{path}")
        with urllib.request.urlopen(req) as r:
            return json.loads(r.read().decode())

    def health(self) -> dict:
        return self._get("/health")

    def verify_audit(self) -> dict:
        return self._get("/audit/verify")

    def execute(self, intent: str, pattern: str) -> dict:
        req = urllib.request.Request(
            f"{self.base_url}/run",
            data=json.dumps({"intent": intent, "pattern": pattern}).encode(),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urllib.request.urlopen(req) as r:
                return json.loads(r.read().decode())
        except urllib.error.HTTPError as e:
            return {"ok": False, "error": str(e)}
