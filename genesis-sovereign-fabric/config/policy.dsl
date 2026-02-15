version: "1.0"
genesis: "sha256:acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a"

scope:
  hardware: false
  network_outbound: false
  fs_write_paths: ["data/", "interventions.jsonl"]

invariante:
  blocked_patterns:
    - "rm -rf"
    - "DROP TABLE"
    - "DELETE FROM"
    - "format"
  deny_on_match: true

rules:
  - name: "critical_infrastructure"
    when: intent == "DECIDE_GRID"
    require: [policy_check, symbol_lookup, audit_append]
    llm_allowed: false

  - name: "assisted_decision"
    when: intent == "ASSIST"
    require: [policy_check, symbol_lookup, adapter_call, output_validation, audit_append]
    llm_allowed: true
    temperature_max: 0.0
