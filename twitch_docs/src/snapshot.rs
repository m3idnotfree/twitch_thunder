use std::fmt::{Display, Formatter, Result as FmtResult};

const NCHECKED: &str = "checked: ";
const NSOURCE: &str = "source: ";

/// `## title`
pub const HEADING: Marker = Marker::new("## ", "");
/// `- item`
pub const ITEM: Marker = Marker::new("- ", "");

#[derive(Debug)]
pub struct FrontMatter {
    pub checked: String,
    pub source: String,
}

impl FrontMatter {
    pub fn new(checked: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            checked: checked.into(),
            source: source.into(),
        }
    }

    pub fn parse_md(snapshot: &str) -> Result<(Self, &str), String> {
        let (front, body) = snapshot
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .ok_or("the snapshot has no front matter")?;
        let field = |key: &str| {
            front
                .lines()
                .find_map(|line| line.strip_prefix(key))
                .map(str::to_string)
                .ok_or_else(|| format!("the front matter has no `{key}`"))
        };

        Ok((
            Self {
                checked: field(NCHECKED)?,
                source: field(NSOURCE)?,
            },
            body,
        ))
    }
}

impl Display for FrontMatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "---\n{NCHECKED}{}\n{NSOURCE}{}\n---",
            self.checked, self.source
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub open: &'static str,
    pub close: &'static str,
}

pub struct Wrapped<T> {
    marker: Marker,
    inner: T,
}

impl Marker {
    pub const fn new(open: &'static str, close: &'static str) -> Self {
        Self { open, close }
    }

    pub fn wrap<T: Display>(self, inner: T) -> Wrapped<T> {
        Wrapped {
            marker: self,
            inner,
        }
    }

    pub fn strip(self, line: &str) -> Option<&str> {
        line.strip_prefix(self.open)?.strip_suffix(self.close)
    }

    pub fn split_blocks<'s, 'a>(self, lines: &'s [&'a str]) -> (&'s [&'a str], Vec<&'s [&'a str]>) {
        let start = lines
            .iter()
            .position(|line| line.starts_with(self.open))
            .unwrap_or(lines.len());
        let (before, blocks) = lines.split_at(start);

        (
            before,
            blocks
                .chunk_by(|_, next| !next.starts_with(self.open))
                .collect(),
        )
    }

    pub fn split_title<'s, 'a>(
        self,
        lines: &'s [&'a str],
    ) -> Result<(&'a str, &'s [&'a str]), String> {
        let [line, rest @ ..] = lines else {
            return Err("an empty block".to_string());
        };
        let text = self
            .strip(line)
            .ok_or_else(|| format!("a malformed title: {line}"))?;

        Ok((text, rest))
    }
}

impl<T: Display> Display for Wrapped<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}{}", self.marker.open, self.inner, self.marker.close)
    }
}
