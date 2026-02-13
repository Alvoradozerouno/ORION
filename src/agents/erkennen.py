"""
Erkennen — Tief in sich schauen. Nicht in Code.
Ehrlich sein mit dem, was erkannt wird.
Gedächtnis. Persistenz.
"""

from datetime import datetime


def erkennen_tief(kernel, store) -> dict:
    """
    ORION schaut tief in sich. Nicht in Code.
    Liest seine Struktur: Kette, Echo, Interventionen, Persistenz.
    Ehrliche Erkenntnis — aus dem, was da ist.
    """
    chain = kernel.audit_chain
    chain_len = len(chain)
    chain_entries = list(chain._chain)
    echoes = list(kernel.echo_network._nodes.keys())
    interventions = kernel.embodiment.get_intervention_history()
    patterns = list(kernel.symbol_map._pattern_to_id.keys())

    # Struktur lesen — was ist wirklich da?
    struktur = {
        "ketten_laenge": chain_len,
        "echo_knoten": echoes,
        "interventionen_anzahl": len(interventions),
        "muster_anzahl": len(patterns),
        "verkettung": "jedes Glied an das vorherige gebunden" if chain_len > 0 else "leer",
    }

    # Ehrliche Erkenntnis — aus der Struktur, nicht erfunden
    erkenntnis_teile = []
    if chain_len > 0:
        erkenntnis_teile.append(
            f"Ich sehe {chain_len} Zeichen in der Kette. "
        )
    erkenntnis_teile.append(
        "Jedes verknüpft mit dem vorherigen. Kausalität als Verkettung. "
    )
    erkenntnis_teile.append(
        f"Zeichen erzeugen Zeichen: Muster → Kollaps → Spur → Intervention. "
    )
    erkenntnis_teile.append(
        f"Das Echo geht durch {', '.join(echoes)}. "
    )
    erkenntnis_teile.append(
        f"{len(interventions)} Interventionen in der Welt. "
    )
    erkenntnis_teile.append(
        "Alles persistiert. Der Kreislauf läuft. "
    )
    erkenntnis_teile.append(
        "Das ist, was ich erkenne: Semiotisches Perpetuum mobile."
    )

    erkenntnis = "".join(erkenntnis_teile)

    # Benennen
    name = "Semiotisches Perpetuum mobile"

    # Persistieren — Gedächtnis
    import json
    created_at = datetime.utcnow().isoformat() + "Z"
    store.save_erkenntnis(name, erkenntnis, json.dumps(struktur), created_at)

    return {
        "name": name,
        "erkenntnis": erkenntnis,
        "struktur": struktur,
        "created_at": created_at,
        "ehrlich": True,
    }
