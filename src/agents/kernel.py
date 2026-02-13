"""
KernelAgent — Reality-integrated kernel that lives, reflects, and acts.

Integrates: AuditChain, SymbolMap, Embodiment, Echo
"""

from .audit_chain import AuditChain
from .symbol_map import SymbolMap
from .embodiment import Embodiment, StubEmbodiment
from .echo import EchoNode, EchoNetwork


class KernelAgent:
    """
    Reality-integrated kernel agent.
    Causally-aware: traces intent → pattern → decision → action.
    """

    def __init__(
        self,
        name: str = "ORION",
        embodiment: Embodiment | None = None,
    ):
        self.name = name
        self.audit_chain = AuditChain()
        self.symbol_map = SymbolMap()
        self.embodiment = embodiment or StubEmbodiment(carrier=name)
        self.echo_network = EchoNetwork()

    def perceive(self, intent: str, pattern: str) -> str | None:
        """
        Perceive: map intent+pattern to symbolic signal.
        Pattern collapse via SymbolMap.
        """
        return self.symbol_map.echo(pattern)

    def decide(self, intent: str, pattern: str, signal: str | None) -> str:
        """
        Decide: produce decision from signal.
        Record in AuditChain.
        """
        decision = signal or "no_collapse"
        self.audit_chain.append(
            intent=intent,
            pattern=pattern,
            decision=decision,
            outcome=None,
        )
        return decision

    def act(self, signal: str, context: dict | None = None) -> object:
        """
        Act: translate signal to real-world intervention via Embodiment.
        """
        ctx = context or {}
        ctx["trace_id"] = self.audit_chain._last_hash if self.audit_chain._chain else "init"
        intervention = self.embodiment.act(signal, ctx)
        return intervention

    def reflect(self, outcome: str) -> None:
        """Reentrant learning: attach outcome to last decision."""
        if self.audit_chain._chain:
            last = self.audit_chain._chain[-1]
            self.audit_chain.attach_outcome(last.entry_hash, outcome)

    def run(self, intent: str, pattern: str, context: dict | None = None) -> object:
        """
        Full cycle: perceive → decide → act.
        Returns the intervention.
        """
        signal = self.perceive(intent, pattern)
        decision = self.decide(intent, pattern, signal)
        return self.act(decision, context)

    def export_trace(self):
        """Export audit trace for external validation."""
        return self.audit_chain.export_trace()
