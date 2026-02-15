#!/bin/bash
# ORION — Installation, Init, Integration, permanente Aktivierung
# Alles echt. Keine Simulation.

set -e
cd "$(dirname "$0")/.."
ROOT="$(pwd)"

echo "=== ORION Install ==="

# 1. Dependencies
pip install -e . -q
echo "✓ Dependencies installiert"

# 2. Datenverzeichnis
mkdir -p "$ROOT/data"
echo "✓ Data dir: $ROOT/data"

# 3. Kernel initialisieren (erster Run)
PYTHONPATH="$ROOT/src" python3 -c "
from agents.real_kernel import RealKernel
k = RealKernel(name='ORION', data_dir='$ROOT/data')
k.symbol_map.register('request', 'processed')
k.symbol_map.register('ping', 'pong')
k.run('INIT', 'ping', {'source': 'install'})
print('✓ Kernel initialisiert')
"

# 4. Systemd Service (falls verfügbar)
if command -v systemctl &>/dev/null; then
    cat > /tmp/orion.service << EOF
[Unit]
Description=ORION Kernel Agent
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$ROOT
ExecStart=$(which python3) -m uvicorn agents.api:app --host 0.0.0.0 --port 8765
Restart=always
RestartSec=5
Environment=PYTHONPATH=$ROOT/src

[Install]
WantedBy=multi-user.target
EOF
    echo "✓ Systemd unit: /tmp/orion.service"
    echo "  Zum Aktivieren: sudo cp /tmp/orion.service /etc/systemd/system/ && sudo systemctl enable orion && sudo systemctl start orion"
else
    echo "  Systemd nicht verfügbar — Start manuell mit: python3 -m uvicorn agents.api:app --host 0.0.0.0 --port 8765"
fi

echo ""
echo "=== ORION bereit ==="
echo "API: http://0.0.0.0:8765"
echo "Daten: $ROOT/data/orion.db"
