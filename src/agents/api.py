"""
API — Echte HTTP-Schnittstelle. Reale Anfragen, reale Antworten.
Keine Simulation.
"""

import os
from contextlib import asynccontextmanager
from pathlib import Path

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

APP_DIR = Path(__file__).resolve().parent.parent.parent / "app"

# RealKernel als Singleton — persistent über alle Requests
_kernel = None
DATA_DIR = Path(os.environ.get("ORION_DATA_DIR", "/workspace/data"))


def get_kernel():
    global _kernel
    if _kernel is None:
        from .real_kernel import RealKernel
        _kernel = RealKernel(name="ORION", data_dir=DATA_DIR)
        # Reale Patterns registrieren
        _kernel.symbol_map.register("request", "processed")
        _kernel.symbol_map.register("ping", "pong")
        _kernel.symbol_map.register("intent", "acknowledged")
    return _kernel


@asynccontextmanager
async def lifespan(app: FastAPI):
    get_kernel()
    yield
    # Cleanup wenn Server stoppt


app = FastAPI(title="ORION API", lifespan=lifespan)


class RunRequest(BaseModel):
    intent: str
    pattern: str
    context: dict | None = None


class SpeakRequest(BaseModel):
    question: str = ""


class ReflectRequest(BaseModel):
    outcome: str


class ExploreRequest(BaseModel):
    topic: str | None = None


@app.get("/")
def root():
    return {"service": "ORION", "status": "active", "real": True, "app": "/app/"}


@app.get("/health")
def health():
    k = get_kernel()
    return {"status": "ok", "trace_entries": len(k.audit_chain)}


@app.post("/run")
def run(req: RunRequest):
    k = get_kernel()
    k._voice_enabled = False
    intervention = k.run(req.intent, req.pattern, req.context)
    return {
        "intervention": {
            "signal": intervention.signal,
            "action_type": intervention.action_type,
            "trace_id": intervention.trace_id,
        },
        "trace_entries": len(k.audit_chain),
    }


@app.post("/reflect")
def reflect(req: ReflectRequest):
    k = get_kernel()
    k.reflect(req.outcome)
    return {"ok": True}


@app.post("/speak")
def speak(req: SpeakRequest):
    k = get_kernel()
    return k.speak(req.question)


@app.get("/trace")
def trace():
    k = get_kernel()
    return {"trace": k.export_trace()}


@app.get("/app_vision")
def app_vision():
    """ORIONs Vorstellung von seiner App."""
    k = get_kernel()
    return k.app_vision()


@app.get("/dna")
def dna():
    """ORION erkennt seine eingebettete DNA. Nicht Code — in ihm."""
    k = get_kernel()
    return k.dna()


@app.post("/explore")
def explore(req: ExploreRequest = ExploreRequest()):
    """Deep Science. Postsynthetisch. Atemporal. Postalgorith."""
    k = get_kernel()
    from agents.exploration import DEEP_SCIENCE_TOPIC
    return k.explore(req.topic or DEEP_SCIENCE_TOPIC)


# ORION App — nach seinen Vorstellungen
if APP_DIR.exists():
    app.mount("/app", StaticFiles(directory=str(APP_DIR), html=True), name="orion_app")


def main():
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8765)


if __name__ == "__main__":
    main()
