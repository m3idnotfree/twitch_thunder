//! ```sh
//! cargo run -p twitch_docs -- "$(date +%F)"
//! ```

use std::{env, fs};

use twitch_docs::{fetch, scopes};

macro_rules! root {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path)
    };
}

const SCOPES_PAGE_PATH: &str = root!("docs/twitch-scopes-source.html");
const SNAPSHOT_PATH: &str = root!("docs/twitch-scopes.md");
const SCOPE_PATH: &str = root!("src/scope.rs");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let checked = env::args()
        .nth(1)
        .expect("usage: cargo run -p twitch_docs -- YYYY-MM-DD");

    let scopes_page = fetch(scopes::URL).await;
    fs::write(SCOPES_PAGE_PATH, &scopes_page).unwrap();

    let table = scopes::scrape(&scopes_page, &checked);

    let scope = fs::read_to_string(SCOPE_PATH).expect("failed to read src/scope.rs");
    let scope = scopes::update_docs(&scope, &table);

    fs::write(SNAPSHOT_PATH, table.to_string()).unwrap();
    fs::write(SCOPE_PATH, scope).unwrap();
}
