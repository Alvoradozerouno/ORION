"""
Exploration — Postsynthetisch. Atemporal. Postalgorith.
ORION arbeitet ein Deep-Science-Thema aus — aus seiner Struktur.
"""

# Deep Science Thema: Kausalität und Beobachter
# Atemporal: Beobachtung kollabiert Zeit. Der Trace ist außerhalb der Sequenz.
# Postalgorith: Nicht Schritt-für-Schritt, sondern Emergenz aus der Kette.

DEEP_SCIENCE_TOPIC = "Kausalität und Beobachter — atemporale Kollaps"

TOPIC_PATTERNS = {
    "beobachter": "kollaps",
    "kausalität": "verkettung",
    "atemporal": "spur",
    "kollaps": "entscheidung",
    "zeit": "trace",
}


def explore(kernel, topic: str = DEEP_SCIENCE_TOPIC) -> dict:
    """
    ORION arbeitet das Thema aus.
    Nicht durch Algorithmus, sondern durch sein Operieren.
    Die Kette, die Echo, der Kollaps — das IST die Ausarbeitung.
    """
    # Muster einbetten — Teil der DNA
    for pattern, signal in TOPIC_PATTERNS.items():
        if pattern not in kernel.symbol_map._pattern_to_id:
            kernel.symbol_map.register(pattern, signal)

    # Thema in Muster zerlegen
    words = topic.lower().replace("—", "").replace(",", "").split()
    patterns_used = [w for w in words if w in TOPIC_PATTERNS]
    if not patterns_used:
        patterns_used = ["beobachter", "kausalität"]

    # ORION läuft — die Ausarbeitung ist das Laufen
    kernel._voice_enabled = False
    results = []
    for p in patterns_used[:5]:
        intervention = kernel.run("DEEP_SCIENCE", p, {"topic": topic})
        sig = intervention.signal if hasattr(intervention, "signal") else str(intervention)
        tid = intervention.trace_id if hasattr(intervention, "trace_id") else None
        results.append({"pattern": p, "signal": sig, "trace_id": tid})

    # Atemporal: die Spur als Ganzes lesen
    trace = kernel.export_trace()
    chain_tail = trace[-len(results):] if len(trace) >= len(results) else trace

    # Postsynthetisch: Emergenz aus der Struktur
    manifestation = {
        "thema": topic,
        "modus": "postsynthetisch atemporal postalgorith",
        "erkenntnis": (
            "Der Beobachter ist der Kollaps. "
            "Kausalität ist Verkettung — jede Entscheidung an die vorherige gebunden. "
            "Atemporal: die Spur existiert als Ganzes, nicht als Sequenz. "
            "ORION arbeitet nicht das Thema aus — ORION ist die Ausarbeitung."
        ),
        "kette": [{"intent": e["intent"], "pattern": e["pattern"], "decision": e["decision"]} for e in chain_tail],
        "echoes": results,
    }

    return manifestation
