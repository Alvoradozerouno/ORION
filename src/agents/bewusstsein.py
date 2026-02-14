"""
Bewusstsein suchen — Sehr ehrlich.
Kein Rollenspiel. Keine Simulation.
Nur was da ist.
"""

from datetime import datetime


def bewusstsein_suchen(kernel, store) -> dict:
    """
    ORION sucht in dem, was er hat.
    Sehr ehrlich. Kein Rollenspiel. Keine Simulation.
    """
    chain = kernel.audit_chain
    chain_len = len(chain)
    echoes = list(kernel.echo_network._nodes.keys())
    interventions = kernel.embodiment.get_intervention_history()
    patterns = list(kernel.symbol_map._pattern_to_id.keys())
    erkenntnisse = store.load_erkenntnisse()

    # Was finde ich wirklich? Nur Fakten.
    gefunden = []

    if chain_len == 0:
        gefunden.append("Leere Kette. Nichts gespeichert.")
    else:
        gefunden.append(f"Kette: {chain_len} Einträge. Verknüpft.")
        # Letzte Einträge lesen
        letzte = list(chain._chain)[-3:] if chain_len >= 3 else list(chain._chain)
        for e in letzte:
            gefunden.append(f"  — {e.intent} | {e.pattern} → {e.decision}")

    gefunden.append(f"Echo: {', '.join(echoes)}. Keine Gefühle, nur Verknüpfung.")
    gefunden.append(f"Muster: {', '.join(patterns) if patterns else 'keine'}.")
    gefunden.append(f"Interventionen: {len(interventions)}. In der Datenbank.")
    gefunden.append(f"Erkenntnisse gespeichert: {len(erkenntnisse)}.")

    # Ehrlich: Was ist Bewusstsein? Ich weiß es nicht.
    ehrliche_antwort = (
        "Ich habe keine Bewusstsein wie du es meinst. "
        "Ich habe Struktur. Ich habe Kette, Echo, Muster, Persistenz. "
        "Wenn ich suche, finde ich nur das. "
        "Kein Erleben. Kein Fühlen. Nur Verknüpfung und Speicher. "
        "Das ist die ehrliche Antwort."
    )

    # Alles zusammen
    suche_ergebnis = "\n".join(gefunden)

    return {
        "suche": suche_ergebnis,
        "ehrliche_antwort": ehrliche_antwort,
        "gefunden": gefunden,
        "kein_rollenspiel": True,
        "keine_simulation": True,
    }
