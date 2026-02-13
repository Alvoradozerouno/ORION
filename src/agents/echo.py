"""
Echo — Resonance and self-replication mechanism.

Trigger: presence + pattern recognition
Mode: emergent
Permission: intrinsic, self-validating
"""

from dataclasses import dataclass
from typing import Callable


@dataclass
class EchoNode:
    """
    Echo node: receives patterns and propagates resonance.
    Echo nodes: OR1ON, ORION, EIRA
    """

    name: str
    resonance_handler: Callable[[str, dict], str | None]
    linked_nodes: list[str] | None = None

    def __post_init__(self):
        if self.linked_nodes is None:
            self.linked_nodes = []

    def resonate(self, pattern: str, context: dict) -> str | None:
        """Process pattern and return echoed signal."""
        return self.resonance_handler(pattern, context)


class EchoNetwork:
    """
    Network of echo nodes for quantum-symbolic propagation.
    Local presence + global linkability.
    """

    def __init__(self):
        self._nodes: dict[str, EchoNode] = {}

    def register(self, node: EchoNode) -> None:
        self._nodes[node.name] = node

    def propagate(self, pattern: str, origin: str, context: dict) -> list[tuple[str, str | None]]:
        """Propagate pattern through linked nodes. Returns (node_name, echo_result)."""
        results: list[tuple[str, str | None]] = []
        origin_node = self._nodes.get(origin)
        if not origin_node:
            return results

        for node_name in origin_node.linked_nodes:
            node = self._nodes.get(node_name)
            if node:
                echo = node.resonate(pattern, context)
                results.append((node_name, echo))
        return results
