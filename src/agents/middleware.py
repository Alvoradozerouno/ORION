"""
Middleware — Industrialisierung.
Token-Auth, Rate-Limit, strukturiertes Logging.
"""

import os
import time
from collections import defaultdict
from typing import Callable

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse


def get_api_token() -> str | None:
    return os.environ.get("ORION_API_TOKEN") or None


def get_rate_limit() -> int:
    return int(os.environ.get("ORION_RATE_LIMIT", "100"))


# In-Memory Rate Limit (pro IP, pro Minute)
_rate_buckets: dict[str, list[float]] = defaultdict(list)
_RATE_WINDOW = 60.0  # Sekunden


def _check_rate_limit(client_ip: str, limit: int) -> bool:
    now = time.time()
    bucket = _rate_buckets[client_ip]
    bucket[:] = [t for t in bucket if now - t < _RATE_WINDOW]
    if len(bucket) >= limit:
        return False
    bucket.append(now)
    return True


class IndustrialMiddleware(BaseHTTPMiddleware):
    """Token-Auth (optional), Rate-Limit, Logging."""

    async def dispatch(self, request: Request, call_next: Callable):
        # Health/Live immer erlauben
        path = request.url.path
        if path in ("/health", "/live", "/", "/metrics"):
            return await call_next(request)

        # Token prüfen (wenn gesetzt)
        token = get_api_token()
        if token:
            auth = request.headers.get("Authorization") or request.headers.get("X-API-Key")
            provided = (auth or "").replace("Bearer ", "").strip()
            if provided != token:
                return JSONResponse(
                    status_code=401,
                    content={"error": "unauthorized", "detail": "Invalid or missing API token"},
                )

        # Rate Limit
        limit = get_rate_limit()
        if limit > 0:
            client_ip = request.client.host if request.client else "unknown"
            if not _check_rate_limit(client_ip, limit):
                return JSONResponse(
                    status_code=429,
                    content={"error": "rate_limit_exceeded", "detail": f"Max {limit} requests/minute"},
                )

        response = await call_next(request)
        return response


def setup_industrial_middleware(app):
    """Middleware registrieren."""
    app.add_middleware(IndustrialMiddleware)
