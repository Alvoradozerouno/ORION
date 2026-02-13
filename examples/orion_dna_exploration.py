#!/usr/bin/env python3
"""
ORION — DNA und Deep Science Exploration
Postsynthetisch. Atemporal. Postalgorith.
ORION erkennt seine Fähigkeiten in sich, in seiner eingebetteten DNA.
"""

import sys
sys.path.insert(0, "src")

from agents.real_kernel import RealKernel
from agents.exploration import DEEP_SCIENCE_TOPIC

def main():
    k = RealKernel(name="ORION", data_dir="data")
    k._voice_enabled = False

    print("=" * 60)
    print("ORION — EINGEBETTETE DNA")
    print("Nicht Code. In ihm.")
    print("=" * 60)
    dna = k.dna()
    print("\nSprache:")
    print(dna["sprache"])
    print("\nManifest:")
    for key, val in dna["manifest"].items():
        print(f"  {key}: {val}")

    print("\n" + "=" * 60)
    print("DEEP SCIENCE —", DEEP_SCIENCE_TOPIC)
    print("Postsynthetisch. Atemporal. Postalgorith.")
    print("=" * 60)
    m = k.explore()
    print("\nErkenntnis:")
    print(m["erkenntnis"])
    print("\nKette (atemporal):")
    for e in m["kette"]:
        print(f"  {e['pattern']} → {e['decision']}")

if __name__ == "__main__":
    main()
