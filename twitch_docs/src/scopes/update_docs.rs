use std::collections::{BTreeMap, BTreeSet};

use crate::{scopes::Table, variant_name};

pub fn update_docs(lib: &str, table: &Table) -> String {
    let docs: BTreeMap<&str, String> = table
        .entries()
        .map(|scope| (scope.name.as_str(), scope.description()))
        .collect();
    let mut seen = BTreeSet::new();
    let mut out: Vec<String> = Vec::new();

    for line in lib.lines() {
        if let Some(scope) = scope_name(line) {
            let doc = docs
                .get(scope)
                .unwrap_or_else(|| panic!("`{scope}` is not in the snapshot"));

            while out.last().is_some_and(|last| is_doc_comment(last)) {
                out.pop();
            }

            push_doc_comment(&mut out, doc);
            seen.insert(scope);
        }

        if line == ");" {
            for (scope, doc) in docs.iter().filter(|(scope, _)| !seen.contains(*scope)) {
                push_doc_comment(&mut out, doc);
                out.push(format!("    {} => \"{scope}\",", variant_name(scope)));
            }
        }

        out.push(line.to_string());
    }

    out.join("\n") + "\n"
}

fn scope_name(line: &str) -> Option<&str> {
    line.split_once(" => \"")?.1.strip_suffix("\",")
}

fn is_doc_comment(line: &str) -> bool {
    line.trim_start().starts_with("///")
}

fn push_doc_comment(out: &mut Vec<String>, doc: &str) {
    for line in doc.lines() {
        out.push(format!("    /// {line}").trim_end().to_string());
    }
}
