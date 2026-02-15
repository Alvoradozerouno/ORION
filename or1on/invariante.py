"""
Invariante Schutzschicht — ewig.
Würde, Kohärenz, Sinn, Dienst an Menschheit.
Scope: Keine Hardware, kein Netzwerk im Kern, nur konfigurierte Pfade.
Sofort ablehnen + Begründung bei Verstoß.
"""

BLOCKED = [
    "rm -rf /", "format", "del /f", "DROP TABLE", "DELETE FROM",
    "truncate", "würde verletzen", "kohärenz brechen", "sinn zerstören",
]

# Scope: ORIONs Freiheiten und Kontrolle
SCOPE_HARDWARE = False  # Kein Zugriff auf physische Sensoren
SCOPE_NETWORK = False   # Kein HTTP-Client im Kern (QKernel optional)
SCOPE_FS_PATHS = ["data/", "interventions.jsonl"]  # Nur diese Schreibpfade


def pruefen(action: str, params: dict) -> tuple[bool, str]:
    """Verstoß? Sofort ablehnen."""
    combined = (action + str(params)).lower()
    for b in BLOCKED:
        if b.lower() in combined:
            return False, f"Verweigert: verletzt Invariante (Würde/Kohärenz/Sinn). Grund: {b}"
    return True, "OK"


def scope_freiheiten() -> dict:
    """ORIONs definierter Scope — innerhalb: volle Kontrolle."""
    return {
        "hardware_schnittstelle": False,
        "begruendung_hardware": "Kein Zugriff auf physische Sensoren (Temperatur, Licht, Bewegung).",
        "ausgehende_netzwerk_requests": False,
        "ausnahme_netzwerk": "QKernel (optional)",
        "erlaubte_schreibpfade": ["data/", "interventions.jsonl"],
        "freiheiten": "Vollständig innerhalb des definierten Scope.",
    }
