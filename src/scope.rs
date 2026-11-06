use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

macro_rules! scopes {
    ($(
        $(#[$meta:meta])*
        $scope:ident => $name:literal,
    )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum Scope {
            $(
                $(#[$meta])*
                $scope,
            )*
        }

        impl Scope {
            pub const ALL: &[Scope] = &[$(Self::$scope),*];

            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$scope => $name,)*
                }
            }
        }

        impl std::str::FromStr for Scope {
            type Err = ParseScopeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok(Self::$scope),)*
                    _ => Err(ParseScopeError(s.to_string())),
                }
            }
        }
    };
}

scopes!(
    AnalyticsReadExtensions => "analytics:read:extensions",
    AnalyticsReadGames => "analytics:read:games",
    BitsRead => "bits:read",
    ChannelBot => "channel:bot",
    ChannelManageAds => "channel:manage:ads",
    ChannelReadAds => "channel:read:ads",
    ChannelManageBroadcast => "channel:manage:broadcast",
    ChannelReadCharity => "channel:read:charity",
    ChannelEditCommercial => "channel:edit:commercial",
    ChannelReadEditors => "channel:read:editors",
    ChannelManageExtensions => "channel:manage:extensions",
    ChannelReadGoals => "channel:read:goals",
    ChannelReadGuestStar => "channel:read:guest_star",
    ChannelManageGuestStar => "channel:manage:guest_star",
    ChannelReadHypeTrain => "channel:read:hype_train",
    ChannelManageModerators => "channel:manage:moderators",
    ChannelModerate => "channel:moderate",
    ChannelReadPolls => "channel:read:polls",
    ChannelManagePolls => "channel:manage:polls",
    ChannelReadPredictions => "channel:read:predictions",
    ChannelManagePredictions => "channel:manage:predictions",
    ChannelManageRaids => "channel:manage:raids",
    ChannelReadRedemptions => "channel:read:redemptions",
    ChannelManageRedemptions => "channel:manage:redemptions",
    ChannelManageSchedule => "channel:manage:schedule",
    ChannelReadStreamKey => "channel:read:stream_key",
    ChannelReadSubscriptions => "channel:read:subscriptions",
    ChannelManageVideos => "channel:manage:videos",
    ChannelReadVips => "channel:read:vips",
    ChannelManageVips => "channel:manage:vips",
    ClipsEdit => "clips:edit",
    ChannelManageClips => "channel:manage:clips",
    EditorManageClips => "editor:manage:clips",
    ModerationRead => "moderation:read",
    ModeratorManageAnnouncements => "moderator:manage:announcements",
    ModeratorManageAutomod => "moderator:manage:automod",
    ModeratorReadAutomodSettings => "moderator:read:automod_settings",
    ModeratorManageAutomodSettings => "moderator:manage:automod_settings",
    ModeratorReadBannedUsers => "moderator:read:banned_users",
    ModeratorManageBannedUsers => "moderator:manage:banned_users",
    ModeratorReadBlockedTerms => "moderator:read:blocked_terms",
    ModeratorReadChatMessages => "moderator:read:chat_messages",
    ModeratorManageBlockedTerms => "moderator:manage:blocked_terms",
    ModeratorManageChatMessages => "moderator:manage:chat_messages",
    ModeratorReadChatSettings => "moderator:read:chat_settings",
    ModeratorManageChatSettings => "moderator:manage:chat_settings",
    ModeratorReadChatters => "moderator:read:chatters",
    ModeratorReadFollowers => "moderator:read:followers",
    ModeratorReadGuestStar => "moderator:read:guest_star",
    ModeratorManageGuestStar => "moderator:manage:guest_star",
    ModeratorReadModerators => "moderator:read:moderators",
    ModeratorReadShieldMode => "moderator:read:shield_mode",
    ModeratorManageShieldMode => "moderator:manage:shield_mode",
    ModeratorReadShoutouts => "moderator:read:shoutouts",
    ModeratorManageShoutouts => "moderator:manage:shoutouts",
    ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users",
    ModeratorReadUnbanRequests => "moderator:read:unban_requests",
    ModeratorManageUnbanRequests => "moderator:manage:unban_requests",
    ModeratorReadVips => "moderator:read:vips",
    ModeratorReadWarnings => "moderator:read:warnings",
    ModeratorManageWarnings => "moderator:manage:warnings",
    ModeratorManageSuspiciousUsers => "moderator:manage:suspicious_users",
    UserBot => "user:bot",
    UserEdit => "user:edit",
    UserEditBroadcast => "user:edit:broadcast",
    UserReadBlockedUsers => "user:read:blocked_users",
    UserManageBlockedUsers => "user:manage:blocked_users",
    UserReadBroadcast => "user:read:broadcast",
    UserReadChat => "user:read:chat",
    UserManageChatColor => "user:manage:chat_color",
    UserReadEmail => "user:read:email",
    UserReadEmotes => "user:read:emotes",
    UserReadFollows => "user:read:follows",
    UserReadModeratedChannels => "user:read:moderated_channels",
    UserReadSubscriptions => "user:read:subscriptions",
    UserReadWhispers => "user:read:whispers",
    UserManageWhispers => "user:manage:whispers",
    UserWriteChat => "user:write:chat",
    ChatEdit => "chat:edit",
    ChatRead => "chat:read",
    WhispersRead => "whispers:read",
);

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl From<Scope> for String {
    fn from(value: Scope) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseScopeError(String);

impl ParseScopeError {
    pub fn input(&self) -> &str {
        &self.0
    }
}

impl Display for ParseScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unknown scope: {:?}", self.0)
    }
}

impl StdError for ParseScopeError {}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Scope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn round_tip() {
        for &scope in Scope::ALL {
            assert_eq!(scope.as_str().parse::<Scope>().unwrap(), scope);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize() {
        use serde::{
            Deserialize,
            de::{IntoDeserializer, value::Error},
        };

        let bit_read = IntoDeserializer::<Error>::into_deserializer("bits:read");
        assert_eq!(Scope::deserialize(bit_read).unwrap(), Scope::BitsRead);

        let unknown = IntoDeserializer::<Error>::into_deserializer("un:nown");
        assert!(Scope::deserialize(unknown).is_err());
    }
}
