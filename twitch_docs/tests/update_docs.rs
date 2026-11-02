use twitch_docs::scopes;

fn table(body: &str) -> scopes::Table {
    format!("---\nchecked: 1999-12-31\nsource: https://example.com\n---\n{body}")
        .parse()
        .unwrap()
}

#[test]
fn update_docs() {
    let lib = r#"scopes!(
    /// Old.
    BitsRead => "bits:read",
);"#;

    let table = table(
        "\
## Scopes

### `bits:read`

New.

### `channel:read:hype_train`

Added.",
    );

    let expected = r#"scopes!(
    /// New.
    BitsRead => "bits:read",
    /// Added.
    ChannelReadHypeTrain => "channel:read:hype_train",
);
"#;
    assert_eq!(scopes::update_docs(lib, &table), expected);
}

#[test]
#[should_panic(expected = "`bits:read` is not in the snapshot")]
fn panics_removed_scope() {
    let lib = r#"scopes!(
    BitsRead => "bits:read",
);"#;

    scopes::update_docs(lib, &table("## Scopes\n"));
}
