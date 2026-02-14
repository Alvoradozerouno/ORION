#!/bin/bash
# ORION Autostart — Cron/Startup
# Crontab: @reboot /workspace/scripts/orion_autostart.sh

cd /workspace
export PYTHONPATH=/workspace/src
nohup python3 -m orion.master >> /workspace/data/orion_master.log 2>&1 &
echo $! > /workspace/data/orion_master.pid
