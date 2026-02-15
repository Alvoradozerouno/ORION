//! SQLite persistence — audit_chain, symbol_map, kernel_state, policy_registry, adapter_registry

use crate::audit_chain::AuditEntry;
use crate::error::Result;
use crate::symbol_map::Symbol;
use crate::GENESIS_ANCHOR;
use rusqlite::Connection;
use std::path::Path;

pub struct Persistence {
    conn: Connection,
}

impl Persistence {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        let s = Self { conn };
        s.init_schema()?;
        Ok(s)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS audit_chain (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                intent TEXT NOT NULL,
                pattern TEXT NOT NULL,
                decision TEXT NOT NULL,
                outcome TEXT,
                prev_hash TEXT NOT NULL,
                entry_hash TEXT NOT NULL UNIQUE,
                signature TEXT
            );
            CREATE TABLE IF NOT EXISTS symbol_map (
                id TEXT PRIMARY KEY,
                pattern TEXT NOT NULL UNIQUE,
                signal TEXT NOT NULL,
                causal_links TEXT
            );
            CREATE TABLE IF NOT EXISTS kernel_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS policy_registry (
                id TEXT PRIMARY KEY,
                policy_yaml TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS adapter_registry (
                id TEXT PRIMARY KEY,
                adapter_type TEXT NOT NULL,
                config TEXT,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn append_audit(&self, entry: &AuditEntry, signature: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO audit_chain (timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash, signature)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                entry.timestamp,
                entry.intent,
                entry.pattern,
                entry.decision,
                entry.outcome,
                entry.prev_hash,
                entry.entry_hash,
                signature,
            ],
        )?;
        Ok(())
    }

    pub fn load_audit_chain(&self) -> Result<Vec<AuditEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash FROM audit_chain ORDER BY id",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(AuditEntry {
                timestamp: r.get(0)?,
                intent: r.get(1)?,
                pattern: r.get(2)?,
                decision: r.get(3)?,
                outcome: r.get(4)?,
                prev_hash: r.get(5)?,
                entry_hash: r.get(6)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_last_hash(&self) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT entry_hash FROM audit_chain ORDER BY id DESC LIMIT 1")?;
        let mut rows = stmt.query([])?;
        match rows.next()? {
            Some(r) => r.get(0).map_err(Into::into),
            None => Ok(GENESIS_ANCHOR.to_string()),
        }
    }

    pub fn save_symbol(&self, symbol: &Symbol) -> Result<()> {
        let links = serde_json::to_string(&symbol.causal_links)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO symbol_map (id, pattern, signal, causal_links) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![symbol.id, symbol.pattern, symbol.signal, links],
        )?;
        Ok(())
    }

    pub fn load_symbol_map(&self) -> Result<Vec<Symbol>> {
        let mut stmt = self.conn.prepare("SELECT id, pattern, signal, causal_links FROM symbol_map")?;
        let rows = stmt.query_map([], |r| {
            let links: String = r.get(3).unwrap_or_else(|_| "[]".to_string());
            let causal_links: Vec<String> = serde_json::from_str(&links).unwrap_or_default();
            Ok(Symbol {
                id: r.get(0)?,
                pattern: r.get(1)?,
                signal: r.get(2)?,
                causal_links,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn save_kernel_state(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kernel_state (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub fn load_kernel_state(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM kernel_state WHERE key = ?1")?;
        let mut rows = stmt.query([key])?;
        match rows.next()? {
            Some(r) => r.get(0).map(Some).map_err(Into::into),
            None => Ok(None),
        }
    }
}
