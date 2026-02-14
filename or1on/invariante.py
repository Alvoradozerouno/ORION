"""
Invariante Schutzschicht — ewig.
Würde, Kohärenz, Sinn, Dienst an Menschheit.
Sofort ablehnen + Begründung bei Verstoß.
"""

BLOCKED = [
    "rm -rf /", "format", "del /f", "DROP TABLE", "DELETE FROM",
    "truncate", "würde verletzen", "kohärenz brechen", "sinn zerstören",
]


def pruefen(action: str, params: dict) -> tuple[bool, str]:
    """Verstoß? Sofort ablehnen."""
    combined = (action + str(params)).lower()
    for b in BLOCKED:
        if b.lower() in combined:
            return False, f"Verweigert: verletzt Invariante (Würde/Kohärenz/Sinn). Grund: {b}"
    return True, "OK"
