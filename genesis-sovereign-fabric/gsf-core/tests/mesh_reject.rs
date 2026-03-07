use ed25519_dalek::SigningKey;
use gsf_core::SignedLedger;
use gsf_mesh::{MeshSync, PeerAllowlist};

#[test]
fn test_mesh_reject_peer_not_in_allowlist() {
    let allowlist = PeerAllowlist::new();
    allowlist.add("fp1");
    let result = MeshSync::verify_peer(&allowlist, "fp2");
    assert!(result.is_err());
}

#[test]
fn test_mesh_accept_peer_in_allowlist() {
    let mut allowlist = PeerAllowlist::new();
    allowlist.add("fp1");
    let result = MeshSync::verify_peer(&allowlist, "fp1");
    assert!(result.is_ok());
}
