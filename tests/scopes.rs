use std::collections::BTreeSet;

use twitch_docs::{scopes, variant_name};
use twitch_thunder::Scope;

const SNAPSHOT: &str = include_str!("../docs/twitch-scopes.md");

fn snapshot() -> scopes::Table {
    SNAPSHOT.parse().unwrap()
}

#[test]
fn matching_snapshot_format() {
    assert!(
        snapshot().to_string() == SNAPSHOT,
        "`scopes::Table` did not round-tip the snapshot"
    );
}

#[test]
fn matching_scopes() {
    let table = snapshot();
    let snapshot: BTreeSet<&str> = table.entries().map(|s| s.name.as_str()).collect();

    let ours: BTreeSet<&str> = Scope::ALL.iter().map(Scope::as_str).collect();
    let missing: Vec<_> = snapshot.difference(&ours).collect();
    let removed: Vec<_> = ours.difference(&snapshot).collect();

    assert!(
        missing.is_empty() && removed.is_empty(),
        "missing: {missing:?}\n\
        removed: {removed:?}"
    );
}

#[test]
fn matching_docs() {
    let scope = include_str!("../src/scope.rs");

    assert!(
        scopes::update_docs(scope, &snapshot()) == scope,
        "run `cargo run -p twitch_docs -- YYYY-MM-DD`"
    );
}

#[test]
fn matching_variant_name() {
    for scope in Scope::ALL {
        assert_eq!(format!("{scope:?}"), variant_name(scope.as_str()));
    }
}
