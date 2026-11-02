use scraper::{Html, Selector};

use crate::{
    scopes::{Table, URL, table::Group},
    snapshot::FrontMatter,
};

pub fn scrape(html: &str, checked: &str) -> Table {
    let doc = Html::parse_document(html);
    let h2 = Selector::parse("h2").unwrap();

    let groups: Vec<Group> = doc.select(&h2).map(Group::parse_html).collect();

    Table::new(FrontMatter::new(checked, URL), groups)
}
