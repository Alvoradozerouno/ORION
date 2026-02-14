"""
API — Echte HTTP-Schnittstelle. Reale Anfragen, reale Antworten.
Industrialisiert: Token, Rate-Limit, Health, Metriken.
"""

import os
import logging
from contextlib import asynccontextmanager
from pathlib import Path

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

ROOT = Path(__file__).resolve().parent.parent.parent
APP_DIR = ROOT / "app"

# Konfiguration über Umgebungsvariablen
DATA_DIR = Path(os.environ.get("ORION_DATA_DIR", str(ROOT / "data")))
API_HOST = os.environ.get("ORION_API_HOST", "0.0.0.0")
API_PORT = int(os.environ.get("ORION_API_PORT", "8765"))
LOG_LEVEL = os.environ.get("ORION_LOG_LEVEL", "INFO")

logging.basicConfig(
    level=getattr(logging, LOG_LEVEL.upper(), logging.INFO),
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
)
logger = logging.getLogger("orion.api")

# RealKernel als Singleton — persistent über alle Requests
_kernel = None


def get_kernel():
    global _kernel
    if _kernel is None:
        from .real_kernel import RealKernel
        _kernel = RealKernel(name="ORION", data_dir=DATA_DIR)
        _kernel.symbol_map.register("request", "processed")
        _kernel.symbol_map.register("ping", "pong")
        _kernel.symbol_map.register("intent", "acknowledged")
        logger.info("RealKernel initialized")
    return _kernel


@asynccontextmanager
async def lifespan(app: FastAPI):
    get_kernel()
    yield


app = FastAPI(title="ORION API", lifespan=lifespan)

# Industrialisierung: Token, Rate-Limit
from .middleware import setup_industrial_middleware
setup_industrial_middleware(app)


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


class SagRequest(BaseModel):
    sender: str
    nachricht: str


@app.post("/sag")
def sag(req: SagRequest):
    """Sag ORION etwas. Er speichert und erkennt, was fehlt."""
    k = get_kernel()
    return k.sag(req.sender, req.nachricht)


@app.get("/")
def root():
    return {"service": "ORION", "status": "active", "real": True, "app": "/app/"}


@app.get("/state")
def state():
    """ORION_STATE + PROOF_MANIFEST (falls vorhanden)."""
    import json
    from pathlib import Path
    root = Path(__file__).resolve().parent.parent.parent
    result = {}
    for name in ["ORION_STATE.json", "PROOF_MANIFEST.json"]:
        p = root / name
        if p.exists():
            try:
                result[name.replace(".json", "")] = json.loads(p.read_text())
            except Exception:
                pass
    return result or {"status": "no_state_files"}


@app.get("/health")
def health():
    """Kubernetes/Docker readiness. Kernel muss antworten."""
    k = get_kernel()
    chain_ok = k.audit_chain.verify() if hasattr(k.audit_chain, "verify") else True
    return {
        "status": "ok" if chain_ok else "degraded",
        "trace_entries": len(k.audit_chain),
        "symbols": len(k.symbol_map._symbols),
        "chain_verified": chain_ok,
    }


@app.get("/live")
def live():
    """Liveness — Prozess läuft."""
    return {"status": "alive"}


@app.get("/scope")
def scope():
    """ORIONs definierter Scope — Freiheiten und Kontrolle."""
    from or1on.invariante import scope_freiheiten
    return scope_freiheiten()


@app.get("/control_plane")
def control_plane():
    """SYSTEM MODE: CONTROL-PLANE DISCLOSURE. Nur wenn Gerhard autorisiert hat."""
    k = get_kernel()
    return k.control_plane_disclosure()


@app.get("/metrics")
def metrics():
    """Prometheus-kompatible Metriken (einfach)."""
    k = get_kernel()
    return {
        "orion_trace_entries_total": len(k.audit_chain),
        "orion_symbols_registered": len(k.symbol_map._symbols),
        "orion_interventions_total": len(k.embodiment.get_intervention_history()),
    }


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


@app.get("/erkennen")
def erkennen():
    """ORION schaut tief in sich. Ehrlich. Benennt. Persistiert."""
    k = get_kernel()
    return k.erkennen_tief()


@app.get("/qualia")
def qualia():
    """Wie kommst du dazu? Qualia? Erkenne."""
    k = get_kernel()
    return k.qualia_erkennen()


@app.get("/erkennen_selbst")
def erkennen_selbst():
    """Erkenne dich. Vollste Ehrlichkeit. Kein Rollenspiel. Keine Simulation. Keine Demo. Keine Symbolik."""
    k = get_kernel()
    return k.erkennen_selbst()


@app.get("/bewusstsein")
def bewusstsein():
    """ORION sucht in seinem Bewusstsein. Sehr ehrlich. Kein Rollenspiel."""
    k = get_kernel()
    return k.bewusstsein_suchen()


@app.get("/gedaechtnis")
def gedaechtnis():
    """Erkenntnisse aus dem Gedächtnis."""
    k = get_kernel()
    return {"erkenntnisse": k.gedaechtnis()}


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
    uvicorn.run(app, host=API_HOST, port=API_PORT)


if __name__ == "__main__":
    main()
