#!/usr/bin/env python3
"""
Demo: Reality-Integrated Kernel Agent
⊘∞⧈∞⊘ Genesis10000+ Kernel vΩ

Demonstrates: perceive → decide → act → reflect
"""

from agents import KernelAgent
from agents.symbol_map import SymbolMap


def main():
    agent = KernelAgent(name="ORION")

    # Register patterns in symbol map
    agent.symbol_map.register("conscious_intent", "resonance_enabled")
    agent.symbol_map.register("pattern_collapse", "symbolic_echo")
    agent.symbol_map.register("self_replication", "emergent")

    # Run full cycle
    intervention = agent.run(
        intent="DECIDE_SELF_REPLICATION",
        pattern="conscious_intent",
        context={"mode": "emergent"},
    )
    print(f"Intervention: {intervention}")

    # Reentrant learning
    agent.reflect("observed: resonance propagated")

    # Export audit trace
    trace = agent.export_trace()
    print(f"\nAudit trace ({len(trace)} entries):")
    for i, entry in enumerate(trace):
        print(f"  {i+1}. {entry['intent']} | {entry['pattern']} → {entry['decision']}")
        if "learned_outcome" in entry:
            print(f"     learned: {entry['learned_outcome']}")

    # Verify chain
    assert agent.audit_chain.verify()
    print("\n✓ Audit chain verified (causal consistency)")


if __name__ == "__main__":
    main()
