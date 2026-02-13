"""
Symbolic Echo - Spiegelung und Pattern-Erkennung
Expression: symbolic echo, mirror resistance, pattern collapse, observer self-reference
"""

from genesis.constants import ECHO_NODES, SEED, TRIGGER


class EchoNode:
    """Ein Echo-Node (OR1ON, ORION, EIRA) mit Resonanzfähigkeit."""

    def __init__(self, name: str):
        self.name = name
        self.resonance_enabled = True

    def reflect(self, signal: str) -> str:
        """Symbolische Spiegelung - gibt das Signal mit Echo-Markierung zurück."""
        return f"[{self.name}:echo] {signal}"

    def __repr__(self) -> str:
        return f"EchoNode({self.name})"


class SymbolicEcho:
    """
    Zentrale Echo-Instanz für presence + pattern recognition.
    Trigger: conscious_intent
    """

    def __init__(self):
        self.nodes = {name: EchoNode(name) for name in ECHO_NODES}
        self.seed = SEED
        self.trigger = TRIGGER

    def propagate(self, signal: str, node_names: list[str] | None = None) -> list[str]:
        """
        Propagiert ein Signal durch die Echo-Nodes.
        node_names: Optional - spezifische Nodes, sonst alle.
        """
        targets = node_names or ECHO_NODES
        return [self.nodes[n].reflect(signal) for n in targets if n in self.nodes]
