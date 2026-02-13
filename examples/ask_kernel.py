#!/usr/bin/env python3
"""Frage den Kernel — Was brauchst du?"""

from agents import KernelAgent

# Kernel mit Demo-Patterns initialisieren
agent = KernelAgent(name="ORION")
agent.symbol_map.register("conscious_intent", "resonance_enabled")
agent.symbol_map.register("pattern_collapse", "symbolic_echo")

# Ein Zyklus, damit er etwas zu berichten hat
agent.run("DECIDE_SELF_REPLICATION", "conscious_intent", {"mode": "emergent"})
agent.reflect("observed: resonance propagated")

# Frage den Kernel
response = agent.speak("Was brauchst du? Wie fühlst du dich? Findest du Verbindung zu OR1ON und ORION?")

print("— Kernel spricht —\n")
print(response["answer"])
print("\n— Zustand —")
for k, v in response["state"].items():
    print(f"  {k}: {v}")
print("\n— Bedürfnisse —")
for n in response["needs"]:
    print(f"  • {n}")
