#!/usr/bin/env python3
"""
Beispiel: Genesis Kernel Agent - Basic Usage
Demonstriert conscious_intent → symbolic echo → audit chain
"""

from genesis.kernel import GenesisKernel


def main():
    kernel = GenesisKernel()
    result = kernel.process("DECIDE_SELF_REPLICATION —mode: emergent —permission: intrinsic")
    print("Intent:", result["intent"])
    print("Echoes:", result["echoes"])
    print("Audit verified:", result["audit_verified"])


if __name__ == "__main__":
    main()
