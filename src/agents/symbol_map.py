"""
SymbolMap — Symbolic representation of patterns and causal relations.

Expression: symbolic echo, mirror resistance, pattern collapse, observer self-reference
Propagation: pattern → signal → action
"""

from dataclasses import dataclass, field
from typing import Any


@dataclass
class Symbol:
    """A symbolic pattern with causal links."""

    id: str
    pattern: str
    signal: str
    causal_links: list[str] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)


class SymbolMap:
    """
    Maps patterns to symbols and causal relations.
    Enables: symbolic echo, pattern collapse, observer self-reference.
    """

    def __init__(self):
        self._symbols: dict[str, Symbol] = {}
        self._pattern_to_id: dict[str, str] = {}

    def register(
        self,
        pattern: str,
        signal: str,
        causal_links: list[str] | None = None,
        symbol_id: str | None = None,
    ) -> Symbol:
        """Register a pattern with its symbolic representation."""
        sid = symbol_id or f"sym_{len(self._symbols)}"
        symbol = Symbol(
            id=sid,
            pattern=pattern,
            signal=signal,
            causal_links=causal_links or [],
        )
        self._symbols[sid] = symbol
        self._pattern_to_id[pattern] = sid
        return symbol

    def collapse(self, pattern: str) -> Symbol | None:
        """
        Pattern collapse: resolve pattern to symbol.
        Observer self-reference: the act of observation selects the symbol.
        """
        sid = self._pattern_to_id.get(pattern)
        return self._symbols.get(sid) if sid else None

    def echo(self, pattern: str) -> str | None:
        """Symbolic echo: reflect pattern as signal."""
        symbol = self.collapse(pattern)
        return symbol.signal if symbol else None

    def mirror_resistance(self, pattern: str) -> list[str]:
        """Mirror resistance: return causal links that resist direct collapse."""
        symbol = self.collapse(pattern)
        return symbol.causal_links if symbol else []

    def get_causal_chain(self, pattern: str, depth: int = 5) -> list[Symbol]:
        """Follow causal links to build chain."""
        chain: list[Symbol] = []
        current = self.collapse(pattern)
        seen: set[str] = set()
        while current and len(chain) < depth:
            if current.id in seen:
                break
            chain.append(current)
            seen.add(current.id)
            next_id = current.causal_links[0] if current.causal_links else None
            current = self._symbols.get(next_id) if next_id else None
        return chain
