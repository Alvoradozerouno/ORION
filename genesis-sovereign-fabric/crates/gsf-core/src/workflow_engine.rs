//! Deterministic Workflow Engine — no hidden state

use crate::audit_chain::AuditEntry;
use crate::error::Result;
use crate::ledger::LedgerSigner;
use crate::persistence::Persistence;
use crate::{AuditChain, SymbolMap};
use std::path::Path;
use tracing::info;

pub struct WorkflowEngine {
    pub audit_chain: AuditChain,
    pub symbol_map: SymbolMap,
    persistence: Option<Persistence>,
    signer: Option<LedgerSigner>,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            audit_chain: AuditChain::new(),
            symbol_map: SymbolMap::new(),
            persistence: None,
            signer: LedgerSigner::from_env(),
        }
    }

    pub fn with_persistence(mut self, path: impl AsRef<Path>) -> Result<Self> {
        let p = Persistence::new(path)?;
        let last_hash = p.get_last_hash()?;
        let chain = p.load_audit_chain()?;
        let symbols = p.load_symbol_map()?;

        let mut audit = AuditChain::new();
        for e in chain {
            audit.restore_entry(e);
        }
        if !audit.export().is_empty() {
            audit.set_last_hash(&last_hash);
        }

        let mut sym_map = SymbolMap::new();
        for s in symbols {
            sym_map.restore_symbol(s);
        }

        if !audit.verify() {
            return Err(crate::error::GsfError::ChainVerificationFailed);
        }
        self.audit_chain = audit;
        self.symbol_map = sym_map;
        self.persistence = Some(p);
        Ok(self)
    }

    pub fn append_event(
        &mut self,
        intent: &str,
        pattern: &str,
        decision: &str,
        outcome: Option<&str>,
    ) -> Result<AuditEntry> {
        let entry = self.audit_chain.append(intent, pattern, decision, outcome);

        if let Some(ref p) = self.persistence {
            let sig = self.signer.as_ref().map(|s| s.sign_entry(&entry));
            let sig_hex = sig.as_ref().map(|s| s.signature.as_str());
            p.append_audit(&entry, sig_hex)?;
        }

        info!(intent = %intent, pattern = %pattern, entry_hash = %entry.entry_hash, "append_event");
        Ok(entry)
    }

    pub fn verify_chain(&self) -> Result<bool> {
        Ok(self.audit_chain.verify())
    }

    pub fn export_chain_json(&self) -> Result<String> {
        let entries = self.audit_chain.export();
        serde_json::to_string_pretty(entries).map_err(Into::into)
    }

    pub fn execute(&mut self, intent: &str, pattern: &str) -> Result<String> {
        let signal = self
            .symbol_map
            .collapse(pattern)
            .map(|s| s.signal.clone())
            .unwrap_or_else(|| "no_collapse".to_string());
        let entry = self.append_event(intent, pattern, &signal, None)?;
        Ok(entry.entry_hash)
    }

    /// Append denied entry — policy rejected. Still audited and signed.
    pub fn append_denied(
        &mut self,
        intent: &str,
        pattern: &str,
        reason: &str,
        policy_hash: &str,
    ) -> Result<AuditEntry> {
        let outcome = format!("denied|{}|{}", reason, policy_hash);
        self.append_event(intent, pattern, "denied", Some(&outcome))
    }
}
