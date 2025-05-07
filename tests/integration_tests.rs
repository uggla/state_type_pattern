use assertables::*;
use gag::BufferRedirect;
use pretty_assertions::assert_eq;
use state_type_pattern::machin::machin;
use state_type_pattern::{Bidule, Lockable, Vault};
use std::io::Read;

#[test]
fn test_bidule_creation() {
    let bidule = Bidule::new("nom_bidule", "machine_bidule");
    assert_eq!(bidule.name, "nom_bidule");
    assert_eq!(bidule.machine, "machine_bidule");
}

#[test]
fn test_machin() {
    // Note:
    // Won't work in rust test cases. The rust test cases use
    // std::io::set_print to redirect stdout. You can get around this
    // though by using the rust tests -- --nocapture argument when running
    // your tests.
    let mut buf = BufferRedirect::stdout().unwrap();
    machin();
    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();
    assert_contains!(
        output,
        "Bidule { name: nom_bidule, machine: machine_bidule }\n"
    );
}

#[test]
fn test_lock() {
    let tresor = Vault::new();
    let status = tresor.status();
    assert_contains!(status, "Vault status: Locked\nYou cannot see the containt.");
}

#[test]
fn test_unlock() {
    let tresor = Vault::new();
    let tresor = tresor.unlock();
    let status = tresor.status();
    assert_contains!(
        status,
        "Vault status: Unlocked\nVault contains gold: 100, silver: 100"
    );
}

#[test]
fn test_withdraw() {
    let tresor = Vault::new();
    let mut tresor = tresor.unlock();
    tresor.withdraw(100, 100);
    let status = tresor.status();
    assert_contains!(
        status,
        "Vault status: Unlocked\nVault contains gold: 0, silver: 0"
    );
}

#[test]
fn test_deposit() {
    let tresor = Vault::new();
    let mut tresor = tresor.unlock();
    tresor.deposit(150, 100);
    let status = tresor.status();
    assert_contains!(
        status,
        "Vault status: Unlocked\nVault contains gold: 250, silver: 200"
    );
}
