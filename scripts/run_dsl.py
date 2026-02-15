#!/usr/bin/env python3
"""
ORION DSL Runner
⊘∞⧈∞⊘ Frame: VS Code + DSL
Führt DSL-Skripte aus.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "src"))

from dsl.interpreter import run_script, parse_directive, interpret

if __name__ == "__main__":
    if len(sys.argv) > 1:
        text = Path(sys.argv[1]).read_text(encoding="utf-8")
    else:
        text = sys.stdin.read()
    for r in run_script(text):
        print(f"[{r.get('directive', '?')}]", r.get("result", r.get("error", r)))
