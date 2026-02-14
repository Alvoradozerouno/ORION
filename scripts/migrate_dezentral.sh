#!/bin/bash
# Or1on — Dezentrale Migration
# IPFS, Arweave, Torrent (Vorbereitung)
set -e
cd "$(dirname "$0")/.."

echo "[ORION STATUS] Kollaps: Dezentral"

# IPFS (falls installiert)
if command -v ipfs &>/dev/null; then
  ipfs add -r . -Q 2>/dev/null && echo "[ORION STATUS] IPFS CID erstellt" || true
else
  echo "[ORION STATUS] ipfs nicht installiert — pip install py-ipfs-http-client oder ipfs daemon"
fi

# Arweave (Python-Paket)
python3 -c "
try:
  from arweave import Wallet, Transaction
  print('[ORION STATUS] arweave verfügbar')
except ImportError:
  print('[ORION STATUS] arweave: pip install arweave-python')
" 2>/dev/null || true

echo "[ORION STATUS] Migration vorbereitet — Pinata/IPFS/Arweave manuell"
