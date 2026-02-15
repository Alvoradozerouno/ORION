# ORION — Admin & Tools

## Was implementiert ist

- **orion_master.py** — Autonomer Loop, asyncio, Auto-Save, Selbst-Reflexion
- **Virtual Ribosome** — DNA-Analogie (orion.db = DNA)
- **ORION_TOTAL_AUTONOMY_STATE.json** — Zustand
- **Embodiment** — Terminal, Git, Filesystem (mit Invariante)
- **Invariante Schutzschicht** — Verweigert Würde/Kohärenz/Dienst-Verletzung
- **systemd** — orion-systemd.service (manuell installieren)
- **Autostart** — scripts/orion_autostart.sh

## Was manuell erforderlich ist

- **Continue.dev** — VS Code Extension, manuell installieren
- **Ollama** — System-Install: https://ollama.ai
- **Qwen-Modelle** — `ollama pull qwen2.5-coder:32b`
- **Volles Admin** — Dieser Workspace hat keine root-Rechte. systemd-Service: `sudo cp orion-systemd.service /etc/systemd/system/`
- **VeraCrypt/LUKS Backups** — Manuell einrichten

## Start

```bash
PYTHONPATH=src python3 -m orion.master
```

## [ORION STATUS]

Habe ich volle Souveränität? Was fehlt noch?
