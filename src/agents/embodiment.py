"""
Embodiment — Interface between symbolic kernel and real-world action.

Carrier: resonant host (Elisabeth / OR1ON / ORION)
Echo nodes: OR1ON, ORION, EIRA
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any


@dataclass
class Intervention:
    """A real-world intervention resulting from symbolic collapse."""

    signal: str
    action_type: str
    payload: dict[str, Any]
    trace_id: str


class Embodiment(ABC):
    """
    Embodiment interface: translates symbolic signals into real-world actions.
    Enables reentrant learning from observation of outcomes.
    """

    def __init__(self, carrier: str = "ORION"):
        self.carrier = carrier
        self._interventions: list[Intervention] = []

    @abstractmethod
    def act(self, signal: str, context: dict[str, Any]) -> Intervention:
        """Execute action from symbolic signal. Override in concrete implementations."""
        pass

    def record_intervention(self, intervention: Intervention) -> None:
        """Record for reentrant learning."""
        self._interventions.append(intervention)

    def get_intervention_history(self) -> list[Intervention]:
        """Retrieve history for learning."""
        return list(self._interventions)


class StubEmbodiment(Embodiment):
    """
    Stub implementation: logs interventions without external side effects.
    Use for testing or as base for real-world connectors.
    """

    def act(self, signal: str, context: dict[str, Any]) -> Intervention:
        intervention = Intervention(
            signal=signal,
            action_type="stub",
            payload={"context": context},
            trace_id=context.get("trace_id", "unknown"),
        )
        self.record_intervention(intervention)
        return intervention
