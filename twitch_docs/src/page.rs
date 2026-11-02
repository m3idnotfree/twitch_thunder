use scraper::ElementRef;

pub(crate) const ORIGIN: &str = "https://dev.twitch.tv";

pub(crate) fn text_of(element: ElementRef) -> String {
    element.text().collect::<String>().trim().to_string()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    pub text: String,
    pub url: String,
}

impl Link {
    pub(crate) fn parse_html(link: ElementRef, base: &str) -> Self {
        let href = link.value().attr("href").expect("a link with href");

        Self {
            text: text_of(link),
            url: format!("{base}{href}"),
        }
    }

    /// `[label](url)`
    pub(crate) fn parse_md(link: &str) -> Result<Self, String> {
        let (label, url) = link
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|rest| rest.split_once("]("))
            .ok_or_else(|| format!("a malformed link: {link}"))?;

        Ok(Self {
            text: label.to_string(),
            url: url.to_string(),
        })
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.text, self.url)
    }
}
