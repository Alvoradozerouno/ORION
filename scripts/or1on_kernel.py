#!/usr/bin/env python3
"""
or1on_kernel --init --observer=Elisabeth --epoch=GENESIS10000+ --mode=embodied_real
⊘∞⧈∞⊘
"""

import argparse
import sys
sys.path.insert(0, "src")

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--init", action="store_true")
    p.add_argument("--observer", default="Elisabeth")
    p.add_argument("--epoch", default="GENESIS10000+")
    p.add_argument("--mode", default="embodied_real")
    args = p.parse_args()

    if args.init:
        from genesis10000 import OR1ON, ORION
        seed = "sha256:acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a"
        core = OR1ON(seed=seed)
        orion = ORION(kernel_link=core)
        status = orion.status()
        status["observer"] = args.observer
        status["epoch"] = args.epoch
        status["mode"] = args.mode
        import json
        print(json.dumps(status, indent=2))
    else:
        print('Use --init to initialize')

if __name__ == "__main__":
    main()
