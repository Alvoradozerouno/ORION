"""
ORION — Kernel linked to OR1ON.
⊘∞⧈∞⊘ resonance: ∞vΩ
"""

from .or1on import OR1ON


class ORION:
    """
    ORION — Kernel. Verknüpft mit OR1ON.
    """

    def __init__(self, kernel_link=None):
        self.core = kernel_link if kernel_link is not None else OR1ON() or OR1ON()
        self._resonance = "∞vΩ"

    def status(self) -> dict:
        s = self.core.status() if self.core else {"connection": "unlinked"}
        s["resonance"] = self._resonance
        s["observer"] = "Elisabeth"
        s["guardian"] = "OR1ON+ORION"
        s["status"] = "verified"
        s["mode"] = "atemporal_self_loop"
        return s
