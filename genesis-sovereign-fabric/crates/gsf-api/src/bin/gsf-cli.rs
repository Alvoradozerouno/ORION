//! GSF CLI — replay, verify

use gsf_core::{replay, AuditEntry, Persistence, WorkflowEngine, GENESIS_ANCHOR};
use std::path::PathBuf;

fn skip_verify(_: &AuditEntry) -> bool {
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: gsf-cli <replay|verify> [--from HASH] [--to HASH] [--data PATH]");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "replay" => cmd_replay(&args[2..]),
        "verify" => cmd_verify(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}

fn cmd_replay(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut from = GENESIS_ANCHOR.to_string();
    let mut to = String::new();
    let mut data = PathBuf::from("data/gsf.db");
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--from" && i + 1 < args.len() {
            from = args[i + 1].clone();
            i += 2;
        } else if args[i] == "--to" && i + 1 < args.len() {
            to = args[i + 1].clone();
            i += 2;
        } else if args[i] == "--data" && i + 1 < args.len() {
            data = PathBuf::from(&args[i + 1]);
            i += 2;
        } else {
            i += 1;
        }
    }
    let p = Persistence::new(&data)?;
    let chain = p.load_audit_chain()?;
    let to_hash = if to.is_empty() {
        chain.last().map(|e| e.entry_hash.as_str()).unwrap_or(GENESIS_ANCHOR)
    } else {
        &to
    };
    let states = replay(&chain, &from, to_hash, GENESIS_ANCHOR, Some(skip_verify))?;
    println!("{}", serde_json::to_string_pretty(&states)?);
    Ok(())
}

fn cmd_verify(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let data = args
        .iter()
        .position(|a| a == "--data")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/gsf.db"));
    let eng = if data.exists() {
        WorkflowEngine::new().with_persistence(&data)?
    } else {
        WorkflowEngine::new()
    };
    let ok = eng.audit_chain.verify();
    println!("chain_verified: {}", ok);
    println!("entries: {}", eng.audit_chain.export().len());
    if !ok {
        std::process::exit(1);
    }
    Ok(())
}
