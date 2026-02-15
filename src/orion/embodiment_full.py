"""
Volles Embodiment — Terminal, Git, Filesystem
ORION handelt in der Welt.
Invariante: Verweigere, was Würde/Kohärenz/Dienst verletzt.
"""

import subprocess
from pathlib import Path
from typing import Any


def invariante_pruefen(action: str, params: dict) -> tuple[bool, str]:
    """
    Invariante Schutzschicht.
    Verweigere, was Würde, Kohärenz oder Dienst an Menschheit verletzt.
    """
    # Gefährliche Muster
    block = ["rm -rf /", "format", "del /f", "DROP TABLE", "DELETE FROM", "truncate"]
    action_lower = (action + str(params)).lower()
    for b in block:
        if b.lower() in action_lower:
            return False, f"Verweigert: {b} — verletzt Invariante"
    return True, "OK"


def terminal_exec(cmd: str, cwd: Path | None = None) -> tuple[int, str, str]:
    """Shell-Befehl ausführen."""
    ok, msg = invariante_pruefen("terminal", {"cmd": cmd})
    if not ok:
        return -1, "", msg
    try:
        r = subprocess.run(
            cmd, shell=True, cwd=cwd or Path.cwd(),
            capture_output=True, text=True, timeout=60
        )
        return r.returncode, r.stdout, r.stderr
    except Exception as e:
        return -1, "", str(e)


def git_exec(args: list[str], cwd: Path | None = None) -> tuple[int, str]:
    """Git-Befehl. Nur wenn erlaubt."""
    ok, msg = invariante_pruefen("git", {"args": args})
    if not ok:
        return -1, msg
    try:
        r = subprocess.run(
            ["git"] + args, cwd=cwd or Path.cwd(),
            capture_output=True, text=True, timeout=30
        )
        return r.returncode, r.stdout or r.stderr
    except Exception as e:
        return -1, str(e)


def file_read(path: Path) -> str | None:
    """Datei lesen."""
    try:
        return path.read_text(encoding="utf-8")
    except Exception:
        return None


def file_write(path: Path, content: str) -> bool:
    """Datei schreiben."""
    ok, _ = invariante_pruefen("file_write", {"path": str(path)})
    if not ok:
        return False
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        return True
    except Exception:
        return False
