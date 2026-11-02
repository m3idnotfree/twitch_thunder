use std::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};

use scraper::{ElementRef, Selector};

use crate::{
    page::{Link, ORIGIN, text_of},
    scopes::Heading,
    snapshot::{FrontMatter, HEADING, ITEM, Marker},
};

const OPTIONAL: &str = " (optional)";

/// ``### `scope` ``
const SCOPE: Marker = Marker::new("### `", "`");
/// `**kind**`
const SECTION: Marker = Marker::new("**", "**");

pub struct Table {
    pub front_matter: FrontMatter,
    groups: Vec<Group>,
}

impl Table {
    pub(crate) fn new(front_matter: FrontMatter, groups: Vec<Group>) -> Self {
        Self {
            front_matter,
            groups,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.groups.iter().flat_map(|group| &group.entries)
    }
}

impl FromStr for Table {
    type Err = String;

    fn from_str(snapshot: &str) -> Result<Self, Self::Err> {
        let (front_matter, body) = FrontMatter::parse_md(snapshot)?;
        let lines: Vec<&str> = body.lines().filter(|line| !line.is_empty()).collect();
        let (before, groups) = HEADING.split_blocks(&lines);

        if let Some(line) = before.first() {
            return Err(format!("text outside a `{}` heading: {line}", HEADING.open));
        }

        Ok(Self {
            front_matter,
            groups: groups
                .into_iter()
                .map(Group::parse_md)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "{}", self.front_matter)?;

        for group in &self.groups {
            write!(f, "\n{group}\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    heading: String,
    entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    /// first `<td>`, and `<code.highlighter-rouge>`.
    pub name: String,
    /// second `<td>` text.
    summary: String,
    sections: Vec<Section>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    /// `<strong>`
    kind: Kind,
    /// `<a>` under `<strong>`.
    endpoints: Vec<Endpoint>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Endpoint {
    link: Link,
    optional: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Api,
    EventSub,
}

impl Kind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Api => "API",
            Self::EventSub => "EventSub",
        }
    }
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "API" => Ok(Self::Api),
            "EventSub" => Ok(Self::EventSub),
            other => Err(format!("unknown section `{other}`, add it to `Kind`")),
        }
    }
}

impl Group {
    pub(crate) fn parse_html(h2: ElementRef) -> Self {
        let id = h2.value().id().expect("an <h2> with an id");
        let heading = Heading::from_id(id)
            .unwrap_or_else(|| panic!("unknown <h2 id=\"{id}\", add it to `Heading`"));

        let mut below = h2.next_siblings().filter_map(ElementRef::wrap);
        let p = below.next();
        assert!(
            p.is_some_and(|p| p.value().name() == "p"),
            "a <p> below <h2 id=\"{id}\">"
        );
        let table = below
            .next()
            .filter(|table| table.value().name() == "table")
            .unwrap_or_else(|| panic!("a <table> below <h2 id=\"{id}\">"));

        let th = Selector::parse("thead th").unwrap();
        let header: Vec<String> = table.select(&th).map(text_of).collect();
        assert_eq!(
            header,
            heading.columns(),
            "the table header below <h2 id=\"{id}\"> changed"
        );

        let row = Selector::parse("tbody tr").unwrap();
        let entries = table.select(&row).map(Entry::parse_html).collect();

        Self {
            heading: text_of(h2),
            entries,
        }
    }

    /// ## heading
    ///
    /// ### `scope`
    pub(crate) fn parse_md(lines: &[&str]) -> Result<Self, String> {
        let (heading, rest) = HEADING.split_title(lines)?;
        let (before, entries) = SCOPE.split_blocks(rest);

        if let Some(line) = before.first() {
            return Err(format!("`{heading}` has text above its scopes: {line}"));
        }

        Ok(Self {
            heading: heading.to_string(),
            entries: entries
                .into_iter()
                .map(Entry::parse_md)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", HEADING.wrap(&self.heading))?;

        for entry in &self.entries {
            write!(f, "\n\n{entry}")?;
        }

        Ok(())
    }
}

impl Entry {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            summary: String::new(),
            sections: Vec::new(),
        }
    }

    pub(crate) fn parse_html(tr: ElementRef) -> Self {
        let cell = Selector::parse("td").unwrap();
        let code = Selector::parse("code").unwrap();

        let cells: Vec<_> = tr.select(&cell).collect();
        let [name, description] = cells[..] else {
            panic!("a scope row must have two cells");
        };
        let name = name.select(&code).next().expect("a scope name in <code>");

        let mut entry = Self::new(text_of(name));
        entry.add_description(description);

        entry
    }

    /// ### `scope`
    ///
    /// **session**
    pub(crate) fn parse_md(lines: &[&str]) -> Result<Self, String> {
        let (name, rest) = SCOPE.split_title(lines)?;
        let (summary, sections) = SECTION.split_blocks(rest);

        if summary
            .iter()
            .any(|line| ITEM.strip(line).is_some_and(|item| item.starts_with('[')))
        {
            return Err(format!("`{name}` has a link outside a section"));
        }

        Ok(Self {
            name: name.to_string(),
            summary: summary.join("\n\n"),
            sections: sections
                .into_iter()
                .map(Section::parse_md)
                .collect::<Result<_, _>>()?,
        })
    }

    fn add_description(&mut self, cell: ElementRef) {
        let mut nodes = cell.children().peekable();

        while let Some(node) = nodes.next() {
            if let Some(text) = node.value().as_text() {
                self.add_text(text);
            } else if let Some(element) = ElementRef::wrap(node) {
                match element.value().name() {
                    "br" => {}
                    "strong" => {
                        let kind = text_of(element)
                            .parse()
                            .unwrap_or_else(|err: String| panic!("{err}"));
                        self.sections.push(Section::new(kind));
                    }
                    "a" => {
                        let note = nodes
                            .next_if(|next| next.value().is_text())
                            .and_then(|next| next.value().as_text())
                            .map_or("", |text| text.trim());

                        self.sections
                            .last_mut()
                            .expect("a <strong> section above a link")
                            .endpoints
                            .push(Endpoint::parse_html(element, note));
                    }
                    other => panic!("unexpected <{other}> in a scope description"),
                }
            }
        }
    }

    fn add_text(&mut self, text: &str) {
        if text.trim().is_empty() {
            return;
        }

        assert!(
            self.sections.is_empty(),
            "unexpected text in a section: {}",
            text.trim()
        );

        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            if !self.summary.is_empty() {
                self.summary.push_str("\n\n");
            }
            self.summary.push_str(line);
        }
    }

    pub fn description(&self) -> String {
        let mut out = self.summary.clone();

        for section in &self.sections {
            if !out.is_empty() {
                out.push_str("\n\n");
            }
            write!(out, "{section}").unwrap();
        }

        out
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}\n\n{}", SCOPE.wrap(&self.name), self.description())
    }
}

impl Section {
    pub(crate) fn new(kind: Kind) -> Self {
        Self {
            kind,
            endpoints: Vec::new(),
        }
    }

    /// `**title**`
    pub(crate) fn parse_md(lines: &[&str]) -> Result<Self, String> {
        let (kind, rest) = SECTION.split_title(lines)?;

        Ok(Self {
            kind: kind.parse()?,
            endpoints: rest
                .iter()
                .map(|line| Endpoint::parse_md(line))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", SECTION.wrap(self.kind.as_str()))?;

        let mut gap = "\n\n";

        for item in &self.endpoints {
            write!(f, "{gap}{item}")?;
            gap = "\n";
        }

        Ok(())
    }
}

impl Endpoint {
    pub(crate) fn parse_html(link: ElementRef, note: &str) -> Self {
        let optional = match note {
            "" => false,
            note if note == OPTIONAL.trim() => true,
            other => panic!("unexpected text after a link: {other}"),
        };

        Self {
            link: Link::parse_html(link, ORIGIN),
            optional,
        }
    }

    /// `- [label](url) (optional)`
    pub(crate) fn parse_md(line: &str) -> Result<Self, String> {
        let text = ITEM
            .strip(line)
            .ok_or_else(|| format!("not a list item: {line}"))?;
        let end = text
            .find(')')
            .ok_or_else(|| format!("a list item without a link: {line}"))?;
        let (link, note) = text.split_at(end + 1);
        let optional = match note {
            "" => false,
            OPTIONAL => true,
            other => return Err(format!("unexpected text after a link: {other}")),
        };

        Ok(Self {
            link: Link::parse_md(link)?,
            optional,
        })
    }

    fn note(&self) -> &'static str {
        if self.optional { OPTIONAL } else { "" }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            ITEM.wrap(format_args!("{}{}", self.link, self.note()))
        )
    }
}

#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};

    use super::Group;

    #[test]
    fn group_to_markdown() {
        let fragment = Html::parse_fragment(
            r#"<h2 id="twitch-api-and-eventsub-scopes">Twitch API and EventSub scopes</h2>

<p>The following table lists the scopes used by the Twitch API.</p>

<table>
  <thead>
    <tr>
      <th>Scope Name</th>
      <th>Type of Access and Associated Endpoints</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code class="highlighter-rouge">analytics:read:extensions</code></td>
      <td>View analytics data for the Twitch Extensions owned by the authenticated account.<br><br><strong>API</strong><br><a href="/docs/api/reference#get-extension-analytics">Get Extension Analytics</a></td>
    </tr>
    <tr>
      <td><code class="highlighter-rouge">user:read:email</code></td>
      <td>View a user’s email address.<br><br><strong>API</strong><br><a href="/docs/api/reference#get-users">Get Users</a> (optional)<br><a href="/docs/api/reference/#update-user">Update User</a> (optional)<br><br><strong>EventSub</strong><br><a href="/docs/eventsub/eventsub-subscription-types/#userupdate">User Update</a> (optional)</td>
    </tr>
  </tbody>
</table>"#,
        );
        let h2 = Selector::parse("h2").unwrap();
        let h2 = fragment.select(&h2).next().unwrap();

        assert_eq!(
            Group::parse_html(h2).to_string(),
            "\
## Twitch API and EventSub scopes

### `analytics:read:extensions`

View analytics data for the Twitch Extensions owned by the authenticated account.

**API**

- [Get Extension Analytics](https://dev.twitch.tv/docs/api/reference#get-extension-analytics)

### `user:read:email`

View a user’s email address.

**API**

- [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) (optional)
- [Update User](https://dev.twitch.tv/docs/api/reference/#update-user) (optional)

**EventSub**

- [User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userupdate) (optional)"
        );
    }
}
