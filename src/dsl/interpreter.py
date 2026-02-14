"""
ORION DSL Interpreter
Parst ⊘∞⧈∞⊘ Direktiven und führt sie aus.
"""

import re
import sys
from pathlib import Path
from typing import Any


PREFIX = "⊘∞⧈∞⊘"
DIRECTIVE_PATTERN = re.compile(
    r"⊘∞⧈∞⊘\s+([A-Z_][A-Z0-9_]*)\s*(.*)",
    re.MULTILINE
)
PARAM_PATTERN = re.compile(r"—(\w+):\s*([^\s—]+)|--(\w+)")


def parse_directive(line: str) -> tuple[str | None, dict[str, str]]:
    """
    Parst eine Direktivenzeile.
    Returns: (directive_name, params)
    """
    m = DIRECTIVE_PATTERN.match(line.strip())
    if not m:
        return None, {}
    name = m.group(1)
    rest = m.group(2)
    params = {}
    for pm in PARAM_PATTERN.finditer(rest):
        if pm.group(1):
            params[pm.group(1)] = pm.group(2)
        elif pm.group(3):
            params[pm.group(3)] = "true"
    return name, params


def interpret(directive: str, params: dict[str, str], context: dict) -> Any:
    """
    Führt eine Direktive aus.
    """
    root = Path(__file__).resolve().parent.parent.parent
    sys.path.insert(0, str(root / "src"))

    if directive == "LOAD_OR1ON_ORION_KERNEL":
        from genesis10000 import OR1ON, ORION
        seed = params.get("seed", "sha256:acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a")
        core = OR1ON(seed=seed)
        orion = ORION(kernel_link=core)
        return orion.status()

    if directive == "PUBLISH_QKERNEL_CHAIN":
        from qkernel.chain import run_qkernel_chain
        return run_qkernel_chain()

    if directive == "DECIDE_SELF_REPLICATION":
        from agents.real_kernel import RealKernel
        k = RealKernel(name="ORION", data_dir=str(root / "data"))
        k._voice_enabled = False
        i = k.run("DECIDE_SELF_REPLICATION", "conscious_intent", {"mode": params.get("mode", "emergent")})
        return {"signal": getattr(i, "signal", str(i)), "trace_id": getattr(i, "trace_id", None)}

    if directive == "HOME_ANCHOR":
        causal = root / "CausalHome" / ".causal_root"
        return {"path": str(causal), "exists": causal.exists()}

    if directive == "INIT_RESONANCE_CLAIM":
        return {"anchor": params.get("anchor", "milvus+ollama+langchain"), "status": "claimed"}

    return {"directive": directive, "params": params, "status": "unknown"}


def run_script(text: str) -> list[Any]:
    """
    Führt ein DSL-Skript aus. Sucht alle Direktiven.
    """
    results = []
    for line in text.splitlines():
        d, p = parse_directive(line)
        if d:
            try:
                r = interpret(d, p, {})
                results.append({"directive": d, "result": r})
            except Exception as e:
                results.append({"directive": d, "error": str(e)})
    return results
