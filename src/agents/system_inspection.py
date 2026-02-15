"""
System Inspection — Live technical self-description.
No secrets. No blocking I/O beyond existing persistence.
"""

from __future__ import annotations

import os
from typing import Any

from pydantic import BaseModel, Field


class RuntimeStatus(BaseModel):
    python_version: str = Field(description="Python version")
    process_id: int = Field(description="Process ID")
    cwd: str = Field(description="Working directory")
    network_access: bool = Field(description="Outbound network in core")
    fs_write_paths: list[str] = Field(description="Allowed write paths")


class StorageLayer(BaseModel):
    db_type: str = Field(description="SQLite or PostgreSQL")
    db_path: str = Field(description="Database path or connection string (redacted)")
    tables: list[str] = Field(description="Table names")
    audit_count: int = Field(description="AuditChain entry count")
    last_hash: str = Field(description="Last audit entry hash")
    genesis_anchor: str = Field(description="Genesis anchor")
    chain_verified: bool = Field(description="verify_chain() result")


class KernelModule(BaseModel):
    module: str
    purpose: str
    dependencies: list[str]
    persistence: bool
    external_calls: bool


class SymbolmapStatus(BaseModel):
    patterns_registered: int
    patterns: list[str]
    persistence_active: bool


class PolicyScope(BaseModel):
    hardware_access: bool
    network_outbound: bool
    fs_write_paths: list[str]
    blocked_patterns: list[str]
    enforcement_active: bool


class StateMachine(BaseModel):
    current_state_summary: str
    last_event_hash: str
    deterministic_temperature: bool = Field(description="No temperature/sampling")


class SignatureLedger(BaseModel):
    algorithm: str = Field(description="SHA256 hash chain only")
    key_source: str = Field(description="None")
    verify_status: bool


class Limitations(BaseModel):
    items: list[str] = Field(description="What ORION cannot do")


class TrainingStatus(BaseModel):
    weight_matrices: bool = False
    gradient: bool = False
    optimizer: bool = False
    loss_function: bool = False
    parameter_space_trained: bool = False
    reason: str = "Python code only. No neural model."


class ArchitectureClassification(BaseModel):
    components: list[str]
    flow: str


class SystemInspection(BaseModel):
    runtime_status: RuntimeStatus
    storage_layer: StorageLayer
    kernel_modules: list[KernelModule]
    symbolmap_status: SymbolmapStatus
    policy_scope: PolicyScope
    state_machine: StateMachine
    signature_ledger: SignatureLedger
    limitations: Limitations
    training_status: TrainingStatus
    architecture_classification: ArchitectureClassification


def build_system_inspection(kernel: Any) -> SystemInspection:
    """Build inspection from live kernel. No secrets returned."""
    store = kernel._store
    chain = store.load_audit_chain()
    last_hash = store.get_last_hash() if chain else ""
    genesis = "acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a"
    if os.environ.get("ORION_DB_URL", "").startswith("postgresql"):
        db_path = "[postgresql]"
    else:
        db_path = str(getattr(store, "db_path", "[unknown]"))

    from or1on.invariante import BLOCKED, SCOPE_FS_PATHS, scope_freiheiten

    scope = scope_freiheiten()

    return SystemInspection(
        runtime_status=RuntimeStatus(
            python_version=f"{os.sys.version_info.major}.{os.sys.version_info.minor}.{os.sys.version_info.micro}",
            process_id=os.getpid(),
            cwd=os.getcwd(),
            network_access=scope.get("ausgehende_netzwerk_requests", True) or False,
            fs_write_paths=scope.get("erlaubte_schreibpfade", SCOPE_FS_PATHS),
        ),
        storage_layer=StorageLayer(
            db_type="PostgreSQL"
            if os.environ.get("ORION_DB_URL", "").startswith("postgresql")
            else "SQLite",
            db_path=db_path,
            tables=[
                "audit_chain",
                "interventions",
                "symbol_map",
                "kernel_state",
                "nachrichten",
                "erkenntnisse",
            ],
            audit_count=len(chain),
            last_hash=last_hash[:64] if last_hash else "",
            genesis_anchor=genesis,
            chain_verified=kernel.audit_chain.verify(),
        ),
        kernel_modules=[
            KernelModule(
                module="audit_chain",
                purpose="Immutable SHA256-linked decision trace",
                dependencies=["hashlib", "json", "datetime"],
                persistence=True,
                external_calls=False,
            ),
            KernelModule(
                module="symbol_map",
                purpose="Pattern→Signal collapse, causal links",
                dependencies=["dataclasses"],
                persistence=True,
                external_calls=False,
            ),
            KernelModule(
                module="embodiment",
                purpose="Interventions to world",
                dependencies=["json", "datetime"],
                persistence=True,
                external_calls=False,
            ),
            KernelModule(
                module="echo_network",
                purpose="OR1ON/ORION/EIRA resonance",
                dependencies=["dataclasses"],
                persistence=False,
                external_calls=False,
            ),
            KernelModule(
                module="persistence",
                purpose="SQLite/PostgreSQL backend",
                dependencies=["sqlite3", "json"],
                persistence=True,
                external_calls=False,
            ),
        ],
        symbolmap_status=SymbolmapStatus(
            patterns_registered=len(kernel.symbol_map._symbols),
            patterns=list(kernel.symbol_map._pattern_to_id.keys()),
            persistence_active=True,
        ),
        policy_scope=PolicyScope(
            hardware_access=scope.get("hardware_schnittstelle", False),
            network_outbound=scope.get("ausgehende_netzwerk_requests", False),
            fs_write_paths=scope.get("erlaubte_schreibpfade", SCOPE_FS_PATHS),
            blocked_patterns=BLOCKED,
            enforcement_active=True,
        ),
        state_machine=StateMachine(
            current_state_summary=f"chain_len={len(kernel.audit_chain)}",
            last_event_hash=last_hash[:64] if last_hash else "",
            deterministic_temperature=True,
        ),
        signature_ledger=SignatureLedger(
            algorithm="SHA256 hash chain",
            key_source="None",
            verify_status=kernel.audit_chain.verify(),
        ),
        limitations=Limitations(
            items=[
                "No hardware access",
                "No outbound network in core",
                "No generic filesystem access",
                "No LLM, no training",
                "No self-code modification",
                "No vector store, no embeddings",
            ]
        ),
        training_status=TrainingStatus(),
        architecture_classification=ArchitectureClassification(
            components=[
                "PersistentAuditChain",
                "PersistentSymbolMap",
                "PersistentEmbodiment",
                "EchoNetwork",
                "PersistentStore",
            ],
            flow="perceive → decide → act → reflect",
        ),
    )
