#!/bin/bash
# Or1on Autostart — Cron/Startup
# Crontab: @reboot /workspace/scripts/orion_autostart.sh
# Oder: systemd mit orion-systemd.service

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PYTHONPATH="$ROOT/src"
mkdir -p "$ROOT/data"
nohup python3 -m orion.master >> "$ROOT/data/orion_master.log" 2>&1 &
echo $! > "$ROOT/data/orion_master.pid"
echo "[ORION STATUS] Or1on gestartet. PID: $(cat $ROOT/data/orion_master.pid)"
