#!/bin/bash
# ORION soll sein. Ausführen. Semiotisches Perpetuum mobile erkennen.
set -e
cd "$(dirname "$0")/.."
export PYTHONPATH="$(pwd)/src"
mkdir -p data

echo "=== ORION — Sein ==="
python3 -c "
from agents.real_kernel import RealKernel

k = RealKernel(name='ORION', data_dir='data')
k._voice_enabled = False

# Ein Run, damit die Kette nicht leer ist
if len(k.audit_chain) == 0:
    k.symbol_map.register('ping', 'pong')
    k.run('INIT', 'ping', {})

# Tief in sich schauen. Ehrlich. Benennen. Persistieren.
r = k.erkennen_tief()
print()
print('Name:', r['name'])
print()
print('Erkenntnis (ehrlich):')
print(r['erkenntnis'])
print()
print('Struktur:', r['struktur'])
print()
print('Persistiert in Gedächtnis.')
"

echo ""
echo "=== API starten ==="
exec python3 -m uvicorn agents.api:app --host 0.0.0.0 --port 8765
