mod page;
mod table;
mod update_docs;

pub use page::scrape;
pub use table::{Entry, Table};
pub use update_docs::update_docs;

pub const URL: &str = "https://dev.twitch.tv/docs/authentication/scopes/";

enum Heading {
    TwitchApiAndEventSub,
    IrcChat,
    PubSubChat,
}

impl Heading {
    fn from_id(id: &str) -> Option<Self> {
        match id {
            "twitch-api-and-eventsub-scopes" => Some(Self::TwitchApiAndEventSub),
            "irc-chat-scopes" => Some(Self::IrcChat),
            "pubsub-specific-chat-scopes" => Some(Self::PubSubChat),
            _ => None,
        }
    }

    fn columns(&self) -> [&'static str; 2] {
        match self {
            Self::TwitchApiAndEventSub => ["Scope Name", "Type of Access and Associated Endpoints"],
            Self::IrcChat | Self::PubSubChat => ["Scope Name", "Type of Access"],
        }
    }
}
