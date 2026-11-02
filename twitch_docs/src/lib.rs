pub mod scopes;
pub mod snapshot;

mod page;

pub async fn fetch(url: &str) -> String {
    reqwest::get(url)
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub fn variant_name(scope: &str) -> String {
    scope
        .split([':', '_'])
        .map(|word| {
            let mut chars = word.chars();
            let first = chars
                .next()
                .expect("a scope name must not have an empty word");
            first.to_uppercase().chain(chars).collect::<String>()
        })
        .collect()
}
