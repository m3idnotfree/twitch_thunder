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
    /// View analytics data for the Twitch Extensions owned by the authenticated account.
    ///
    /// **API**
    ///
    /// - [Get Extension Analytics](https://dev.twitch.tv/docs/api/reference#get-extension-analytics)
    AnalyticsReadExtensions => "analytics:read:extensions",
    /// View analytics data for the games owned by the authenticated account.
    ///
    /// **API**
    ///
    /// - [Get Game Analytics](https://dev.twitch.tv/docs/api/reference#get-game-analytics)
    AnalyticsReadGames => "analytics:read:games",
    /// View Bits-related products and redemptions for a channel.
    ///
    /// **API**
    ///
    /// - [Get Bits Leaderboard](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
    /// - [Get Custom Power-up](https://dev.twitch.tv/docs/api/reference/#get-custom-power-up)
    ///
    /// **EventSub**
    ///
    /// - [Channel Bits Use](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelbitsuse)
    /// - [Channel Cheer](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer)
    /// - [Channel Custom Power-ups Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcustom_power_up_redemptionadd)
    BitsRead => "bits:read",
    /// Joins your channel’s chatroom as a bot user, and perform chat-related actions as that user.
    ///
    /// **API**
    ///
    /// - [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)
    ///
    /// **EventSub**
    ///
    /// - [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
    /// - [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
    /// - [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
    /// - [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
    /// - [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
    /// - [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
    ChannelBot => "channel:bot",
    /// Manage ads schedule on a channel.
    ///
    /// **API**
    ///
    /// - [Snooze Next Ad](https://dev.twitch.tv/docs/api/reference#snooze-next-ad)
    ChannelManageAds => "channel:manage:ads",
    /// Read the ads schedule and details on your channel.
    ///
    /// **API**
    ///
    /// - [Get Ad Schedule](https://dev.twitch.tv/docs/api/reference#get-ad-schedule)
    ///
    /// **EventSub**
    ///
    /// - [Channel Ad Break Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin)
    ChannelReadAds => "channel:read:ads",
    /// Manage a channel’s broadcast configuration, including updating channel configuration and managing stream markers and stream tags.
    ///
    /// **API**
    ///
    /// - [Modify Channel Information](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    /// - [Create Stream Marker](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
    /// - [Replace Stream Tags](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
    ChannelManageBroadcast => "channel:manage:broadcast",
    /// Read charity campaign details and user donations on your channel.
    ///
    /// **API**
    ///
    /// - [Get Charity Campaign](https://dev.twitch.tv/docs/api/reference#get-charity-campaign)
    /// - [Get Charity Campaign Donations](https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations)
    ///
    /// **EventSub**
    ///
    /// - [Charity Donation](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaigndonate)
    /// - [Charity Campaign Start](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstart)
    /// - [Charity Campaign Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignprogress)
    /// - [Charity Campaign Stop](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstop)
    ChannelReadCharity => "channel:read:charity",
    /// Run commercials on a channel.
    ///
    /// **API**
    ///
    /// - [Start Commercial](https://dev.twitch.tv/docs/api/reference#start-commercial)
    ChannelEditCommercial => "channel:edit:commercial",
    /// View a list of users with the editor role for a channel.
    ///
    /// **API**
    ///
    /// - [Get Channel Editors](https://dev.twitch.tv/docs/api/reference#get-channel-editors)
    ChannelReadEditors => "channel:read:editors",
    /// Manage a channel’s Extension configuration, including activating Extensions.
    ///
    /// **API**
    ///
    /// - [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
    /// - [Update User Extensions](https://dev.twitch.tv/docs/api/reference#update-user-extensions)
    ChannelManageExtensions => "channel:manage:extensions",
    /// View Creator Goals for a channel.
    ///
    /// **API**
    ///
    /// - [Get Creator Goals](https://dev.twitch.tv/docs/api/reference#get-creator-goals)
    ///
    /// **EventSub**
    ///
    /// - [Goal Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalbegin)
    /// - [Goal Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalprogress)
    /// - [Goal End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalend)
    ChannelReadGoals => "channel:read:goals",
    /// Read Guest Star details for your channel.
    ///
    /// **API**
    ///
    /// - [Get Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#get-channel-guest-star-settings)
    /// - [Get Guest Star Session](https://dev.twitch.tv/docs/api/reference#get-guest-star-session)
    /// - [Get Guest Star Invites](https://dev.twitch.tv/docs/api/reference#get-guest-star-invites)
    ///
    /// **EventSub**
    ///
    /// - [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
    /// - [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
    /// - [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
    /// - [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)
    ChannelReadGuestStar => "channel:read:guest_star",
    /// Manage Guest Star for your channel.
    ///
    /// **API**
    ///
    /// - [Update Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#update-channel-guest-star-settings)
    /// - [Create Guest Star Session](https://dev.twitch.tv/docs/api/reference#create-guest-star-session)
    /// - [End Guest Star Session](https://dev.twitch.tv/docs/api/reference#end-guest-star-session)
    /// - [Send Guest Star Invite](https://dev.twitch.tv/docs/api/reference#send-guest-star-invite)
    /// - [Delete Guest Star Invite](https://dev.twitch.tv/docs/api/reference#delete-guest-star-invite)
    /// - [Assign Guest Star Slot](https://dev.twitch.tv/docs/api/reference#assign-guest-star-slot)
    /// - [Update Guest Star Slot](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot)
    /// - [Delete Guest Star Slot](https://dev.twitch.tv/docs/api/reference#delete-guest-star-slot)
    /// - [Update Guest Star Slot Settings](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot-settings)
    ///
    /// **EventSub**
    ///
    /// - [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
    /// - [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
    /// - [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
    /// - [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)
    ChannelManageGuestStar => "channel:manage:guest_star",
    /// View Hype Train information for a channel.
    ///
    /// **API**
    ///
    /// - [Get Hype Train Status](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)
    ///
    /// **EventSub**
    ///
    /// - [Hype Train Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainbegin)
    /// - [Hype Train Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainprogress)
    /// - [Hype Train End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainend)
    ChannelReadHypeTrain => "channel:read:hype_train",
    /// Add or remove the moderator role from users in your channel.
    ///
    /// **API**
    ///
    /// - [Add Channel Moderator](https://dev.twitch.tv/docs/api/reference#add-channel-moderator)
    /// - [Remove Channel Moderator](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator)
    /// - [Get Moderators](https://dev.twitch.tv/docs/api/reference/#get-moderators)
    ChannelManageModerators => "channel:manage:moderators",
    /// Perform moderation actions in a channel.
    ///
    /// **EventSub**
    ///
    /// - [Channel Ban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelban)
    /// - [Channel Unban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban)
    ChannelModerate => "channel:moderate",
    /// View a channel’s polls.
    ///
    /// **API**
    ///
    /// - [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls)
    ///
    /// **EventSub**
    ///
    /// - [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)
    /// - [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)
    /// - [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)
    ChannelReadPolls => "channel:read:polls",
    /// Manage a channel’s polls.
    ///
    /// **API**
    ///
    /// - [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls)
    /// - [Create Poll](https://dev.twitch.tv/docs/api/reference#create-poll)
    /// - [End Poll](https://dev.twitch.tv/docs/api/reference#end-poll)
    ///
    /// **EventSub**
    ///
    /// - [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)
    /// - [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)
    /// - [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)
    ChannelManagePolls => "channel:manage:polls",
    /// View a channel’s Channel Points Predictions.
    ///
    /// **API**
    ///
    /// - [Get Channel Points Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions)
    ///
    /// **EventSub**
    ///
    /// - [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)
    /// - [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)
    /// - [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)
    /// - [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)
    ChannelReadPredictions => "channel:read:predictions",
    /// Manage of channel’s Channel Points Predictions
    ///
    /// **API**
    ///
    /// - [Get Channel Points Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions)
    /// - [Create Channel Points Prediction](https://dev.twitch.tv/docs/api/reference#create-prediction)
    /// - [End Channel Points Prediction](https://dev.twitch.tv/docs/api/reference#end-prediction)
    ///
    /// **EventSub**
    ///
    /// - [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)
    /// - [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)
    /// - [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)
    /// - [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)
    ChannelManagePredictions => "channel:manage:predictions",
    /// Manage a channel raiding another channel.
    ///
    /// **API**
    ///
    /// - [Start a raid](https://dev.twitch.tv/docs/api/reference#start-a-raid)
    /// - [Cancel a raid](https://dev.twitch.tv/docs/api/reference#cancel-a-raid)
    ChannelManageRaids => "channel:manage:raids",
    /// View Channel Points custom rewards and their redemptions on a channel.
    ///
    /// **API**
    ///
    /// - [Get Custom Reward](https://dev.twitch.tv/docs/api/reference#get-custom-reward)
    /// - [Get Custom Reward Redemption](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
    ///
    /// **EventSub**
    ///
    /// - [Channel Points Automatic Reward Redemption](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)
    /// - [Channel Points Automatic Reward Redemption v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd-v2)
    /// - [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)
    /// - [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)
    /// - [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)
    /// - [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)
    /// - [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)
    ChannelReadRedemptions => "channel:read:redemptions",
    /// Manage Channel Points custom rewards and their redemptions on a channel.
    ///
    /// **API**
    ///
    /// - [Get Custom Reward](https://dev.twitch.tv/docs/api/reference#get-custom-reward)
    /// - [Get Custom Reward Redemption](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
    /// - [Create Custom Rewards](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
    /// - [Delete Custom Reward](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
    /// - [Update Custom Reward](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
    /// - [Update Redemption Status](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
    ///
    /// **EventSub**
    ///
    /// - [Channel Points Automatic Reward Redemption](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)
    /// - [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)
    /// - [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)
    /// - [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)
    /// - [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)
    /// - [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)
    ChannelManageRedemptions => "channel:manage:redemptions",
    /// Manage a channel’s stream schedule.
    ///
    /// **API**
    ///
    /// - [Update Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule)
    /// - [Create Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
    /// - [Update Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
    /// - [Delete Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)
    ChannelManageSchedule => "channel:manage:schedule",
    /// View an authorized user’s stream key.
    ///
    /// **API**
    ///
    /// - [Get Stream Key](https://dev.twitch.tv/docs/api/reference#get-stream-key)
    ChannelReadStreamKey => "channel:read:stream_key",
    /// View a list of all subscribers to a channel and check if a user is subscribed to a channel.
    ///
    /// **API**
    ///
    /// - [Get Broadcaster Subscriptions](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
    ///
    /// **EventSub**
    ///
    /// - [Channel Subscribe](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscribe)
    /// - [Channel Subscription End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionend)
    /// - [Channel Subscription Gift](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptiongift)
    /// - [Channel Subscription Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionmessage)
    ChannelReadSubscriptions => "channel:read:subscriptions",
    /// Manage a channel’s videos, including deleting videos.
    ///
    /// **API**
    ///
    /// - [Delete Videos](https://dev.twitch.tv/docs/api/reference#delete-videos)
    ChannelManageVideos => "channel:manage:videos",
    /// Read the list of VIPs in your channel.
    ///
    /// **API**
    ///
    /// - [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips)
    ///
    /// **EventSub**
    ///
    /// - [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)
    /// - [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)
    ChannelReadVips => "channel:read:vips",
    /// Add or remove the VIP role from users in your channel.
    ///
    /// **API**
    ///
    /// - [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips)
    /// - [Add Channel VIP](https://dev.twitch.tv/docs/api/reference#add-channel-vip)
    /// - [Remove Channel VIP](https://dev.twitch.tv/docs/api/reference#remove-channel-vip)
    ///
    /// **EventSub**
    ///
    /// - [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)
    /// - [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)
    ChannelManageVips => "channel:manage:vips",
    /// Manage Clips for a channel.
    ///
    /// **API**
    ///
    /// - [Create Clip](https://dev.twitch.tv/docs/api/reference#create-clip)
    ClipsEdit => "clips:edit",
    /// Manage Clips for a channel.
    ///
    /// **API**
    ///
    /// - [Create Clip From VOD](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod)
    /// - [Get Clips Download](https://dev.twitch.tv/docs/api/reference#get-clips-download)
    ChannelManageClips => "channel:manage:clips",
    /// Manage Clips as an editor.
    ///
    /// **API**
    ///
    /// - [Create Clip From VOD](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod)
    /// - [Get Clips Download](https://dev.twitch.tv/docs/api/reference#get-clips-download)
    EditorManageClips => "editor:manage:clips",
    /// View a channel’s moderation data including Moderators, Bans, Timeouts, and Automod settings.
    ///
    /// **API**
    ///
    /// - [Check AutoMod Status](https://dev.twitch.tv/docs/api/reference#check-automod-status)
    /// - [Get Banned Users](https://dev.twitch.tv/docs/api/reference#get-banned-users)
    /// - [Get Moderators](https://dev.twitch.tv/docs/api/reference#get-moderators)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderator Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatoradd)
    /// - [Channel Moderator Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatorremove)
    ModerationRead => "moderation:read",
    /// Send announcements in channels where you have the moderator role.
    ///
    /// **API**
    ///
    /// - [Send Chat Announcement](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)
    ModeratorManageAnnouncements => "moderator:manage:announcements",
    /// Manage messages held for review by AutoMod in channels where you are a moderator.
    ///
    /// **API**
    ///
    /// - [Manage Held AutoMod Messages](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)
    ///
    /// **EventSub**
    ///
    /// - [AutoMod Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold)
    /// - [AutoMod Message Hold v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold-v2)
    /// - [AutoMod Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate)
    /// - [AutoMod Message Update v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate-v2)
    /// - [AutoMod Terms Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodtermsupdate)
    ModeratorManageAutomod => "moderator:manage:automod",
    /// View a broadcaster’s AutoMod settings.
    ///
    /// **API**
    ///
    /// - [Get AutoMod Settings](https://dev.twitch.tv/docs/api/reference#get-automod-settings)
    ///
    /// **EventSub**
    ///
    /// - [AutoMod Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodsettingsupdate)
    ModeratorReadAutomodSettings => "moderator:read:automod_settings",
    /// Manage a broadcaster’s AutoMod settings.
    ///
    /// **API**
    ///
    /// - [Update AutoMod Settings](https://dev.twitch.tv/docs/api/reference#update-automod-settings)
    ModeratorManageAutomodSettings => "moderator:manage:automod_settings",
    /// Read the list of bans or unbans in channels where you have the moderator role.
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    ModeratorReadBannedUsers => "moderator:read:banned_users",
    /// Ban and unban users.
    ///
    /// **API**
    ///
    /// - [Get Banned Users](https://dev.twitch.tv/docs/api/reference/#get-banned-users)
    /// - [Ban User](https://dev.twitch.tv/docs/api/reference#ban-user)
    /// - [Unban User](https://dev.twitch.tv/docs/api/reference#unban-user)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    ModeratorManageBannedUsers => "moderator:manage:banned_users",
    /// View a broadcaster’s list of blocked terms.
    ///
    /// **API**
    ///
    /// - [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorReadBlockedTerms => "moderator:read:blocked_terms",
    /// Read deleted chat messages in channels where you have the moderator role and get pinned chat messages.
    ///
    /// **API**
    ///
    /// - [Get Pinned Chat Message](https://dev.twitch.tv/docs/api/reference#get-pinned-chat-message)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorReadChatMessages => "moderator:read:chat_messages",
    /// Manage a broadcaster’s list of blocked terms.
    ///
    /// **API**
    ///
    /// - [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
    /// - [Add Blocked Term](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
    /// - [Remove Blocked Term](https://dev.twitch.tv/docs/api/reference#remove-blocked-term)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorManageBlockedTerms => "moderator:manage:blocked_terms",
    /// Delete chat messages in channels where you have the moderator role and manage pinned chat messages.
    ///
    /// **API**
    ///
    /// - [Delete Chat Messages](https://dev.twitch.tv/docs/api/reference#delete-chat-messages)
    /// - [Pin Chat Message](https://dev.twitch.tv/docs/api/reference#pin-chat-message)
    /// - [Update Pinned Chat Message](https://dev.twitch.tv/docs/api/reference#update-pinned-chat-message)
    /// - [Unpin Chat Message](https://dev.twitch.tv/docs/api/reference#unpin-chat-message)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorManageChatMessages => "moderator:manage:chat_messages",
    /// View a broadcaster’s chat room settings.
    ///
    /// **API**
    ///
    /// - [Get Chat Settings](https://dev.twitch.tv/docs/api/reference#get-chat-settings)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorReadChatSettings => "moderator:read:chat_settings",
    /// Manage a broadcaster’s chat room settings.
    ///
    /// **API**
    ///
    /// - [Update Chat Settings](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorManageChatSettings => "moderator:manage:chat_settings",
    /// View the chatters in a broadcaster’s chat room.
    ///
    /// **API**
    ///
    /// - [Get Chatters](https://dev.twitch.tv/docs/api/reference#get-chatters)
    ModeratorReadChatters => "moderator:read:chatters",
    /// Read the followers of a broadcaster.
    ///
    /// **API**
    ///
    /// - [Get Channel Followers](https://dev.twitch.tv/docs/api/reference#get-channel-followers)
    ///
    /// **EventSub**
    ///
    /// - [Channel Follow](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow)
    ModeratorReadFollowers => "moderator:read:followers",
    /// Read Guest Star details for channels where you are a Guest Star moderator.
    ///
    /// **API**
    ///
    /// - [Get Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#get-channel-guest-star-settings)
    /// - [Get Guest Star Session](https://dev.twitch.tv/docs/api/reference#get-guest-star-session)
    /// - [Get Guest Star Invites](https://dev.twitch.tv/docs/api/reference#get-guest-star-invites)
    ///
    /// **EventSub**
    ///
    /// - [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
    /// - [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
    /// - [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
    /// - [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)
    ModeratorReadGuestStar => "moderator:read:guest_star",
    /// Manage Guest Star for channels where you are a Guest Star moderator.
    ///
    /// **API**
    ///
    /// - [Send Guest Star Invite](https://dev.twitch.tv/docs/api/reference#send-guest-star-invite)
    /// - [Delete Guest Star Invite](https://dev.twitch.tv/docs/api/reference#delete-guest-star-invite)
    /// - [Assign Guest Star Slot](https://dev.twitch.tv/docs/api/reference#assign-guest-star-slot)
    /// - [Update Guest Star Slot](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot)
    /// - [Delete Guest Star Slot](https://dev.twitch.tv/docs/api/reference#delete-guest-star-slot)
    /// - [Update Guest Star Slot Settings](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot-settings)
    ///
    /// **EventSub**
    ///
    /// - [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
    /// - [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
    /// - [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
    /// - [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)
    ModeratorManageGuestStar => "moderator:manage:guest_star",
    /// Read the list of moderators in channels where you have the moderator role.
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    ModeratorReadModerators => "moderator:read:moderators",
    /// View a broadcaster’s Shield Mode status.
    ///
    /// **API**
    ///
    /// - [Get Shield Mode Status](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status)
    ///
    /// **EventSub**
    ///
    /// - [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)
    /// - [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)
    ModeratorReadShieldMode => "moderator:read:shield_mode",
    /// Manage a broadcaster’s Shield Mode status.
    ///
    /// **API**
    ///
    /// - [Update Shield Mode Status](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status)
    ///
    /// **EventSub**
    ///
    /// - [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)
    /// - [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)
    ModeratorManageShieldMode => "moderator:manage:shield_mode",
    /// View a broadcaster’s shoutouts.
    ///
    /// **EventSub**
    ///
    /// - [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)
    /// - [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)
    ModeratorReadShoutouts => "moderator:read:shoutouts",
    /// Manage a broadcaster’s shoutouts.
    ///
    /// **API**
    ///
    /// - [Send a Shoutout](https://dev.twitch.tv/docs/api/reference#send-a-shoutout)
    ///
    /// **EventSub**
    ///
    /// - [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)
    /// - [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)
    ModeratorManageShoutouts => "moderator:manage:shoutouts",
    /// Read chat messages from suspicious users and see users flagged as suspicious in channels where the user has the moderator role.
    ///
    /// **EventSub**
    ///
    /// - [Channel Suspicious User Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_usermessage)
    /// - [Channel Suspicious User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_userupdate)
    ModeratorReadSuspiciousUsers => "moderator:read:suspicious_users",
    /// View a broadcaster’s unban requests.
    ///
    /// **API**
    ///
    /// - [Get Unban Requests](https://dev.twitch.tv/docs/api/reference/#get-unban-requests)
    ///
    /// **EventSub**
    ///
    /// - [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)
    /// - [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorReadUnbanRequests => "moderator:read:unban_requests",
    /// Manage a broadcaster’s unban requests.
    ///
    /// **API**
    ///
    /// - [Resolve Unban Requests](https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests)
    ///
    /// **EventSub**
    ///
    /// - [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)
    /// - [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ModeratorManageUnbanRequests => "moderator:manage:unban_requests",
    /// Read the list of VIPs in channels where you have the moderator role.
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    ModeratorReadVips => "moderator:read:vips",
    /// Read warnings in channels where you have the moderator role.
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    /// - [Channel Warning Acknowledge](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)
    /// - [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend)
    ModeratorReadWarnings => "moderator:read:warnings",
    /// Warn users in channels where you have the moderator role.
    ///
    /// **API**
    ///
    /// - [Warn Chat User](https://dev.twitch.tv/docs/api/reference#warn-chat-user)
    ///
    /// **EventSub**
    ///
    /// - [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    /// - [Channel Warning Acknowledge](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)
    /// - [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend)
    ModeratorManageWarnings => "moderator:manage:warnings",
    /// Manage suspicious user statuses in channels where the user has the moderator role.
    ///
    /// **API**
    ///
    /// - [Add suspicious status to chat user](https://dev.twitch.tv/docs/api/reference/#add-suspicious-status-to-chat-user)
    /// - [Remove suspicious status from chat user](https://dev.twitch.tv/docs/api/reference/#remove-suspicious-status-from-chat-user)
    ModeratorManageSuspiciousUsers => "moderator:manage:suspicious_users",
    /// Join a specified chat channel as your user and appear as a bot, and perform chat-related actions as your user.
    ///
    /// **API**
    ///
    /// - [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)
    ///
    /// **EventSub**
    ///
    /// - [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
    /// - [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
    /// - [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
    /// - [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
    /// - [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
    /// - [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
    /// - [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_user_message_hold)
    /// - [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)
    UserBot => "user:bot",
    /// Manage a user object.
    ///
    /// **API**
    ///
    /// - [Update User](https://dev.twitch.tv/docs/api/reference#update-user)
    UserEdit => "user:edit",
    /// View and edit a user’s broadcasting configuration, including Extension configurations.
    ///
    /// **API**
    ///
    /// - [Get User Extensions](https://dev.twitch.tv/docs/api/reference/#get-user-extensions)
    /// - [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions)
    /// - [Update User Extensions](https://dev.twitch.tv/docs/api/reference/#update-user-extensions)
    UserEditBroadcast => "user:edit:broadcast",
    /// View the block list of a user.
    ///
    /// **API**
    ///
    /// - [Get User Block List](https://dev.twitch.tv/docs/api/reference#get-user-block-list)
    UserReadBlockedUsers => "user:read:blocked_users",
    /// Manage the block list of a user.
    ///
    /// **API**
    ///
    /// - [Block User](https://dev.twitch.tv/docs/api/reference#block-user)
    /// - [Unblock User](https://dev.twitch.tv/docs/api/reference#unblock-user)
    UserManageBlockedUsers => "user:manage:blocked_users",
    /// View a user’s broadcasting configuration, including Extension configurations.
    ///
    /// **API**
    ///
    /// - [Get Stream Markers](https://dev.twitch.tv/docs/api/reference#get-stream-markers)
    /// - [Get User Extensions](https://dev.twitch.tv/docs/api/reference#get-user-extensions)
    /// - [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
    UserReadBroadcast => "user:read:broadcast",
    /// Receive chatroom messages and informational notifications relating to a channel’s chatroom.
    ///
    /// **EventSub**
    ///
    /// - [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
    /// - [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
    /// - [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
    /// - [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
    /// - [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
    /// - [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
    /// - [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_hold)
    /// - [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)
    UserReadChat => "user:read:chat",
    /// Update the color used for the user’s name in chat.
    ///
    /// **API**
    ///
    /// - [Update User Chat Color](https://dev.twitch.tv/docs/api/reference#update-user-chat-color)
    UserManageChatColor => "user:manage:chat_color",
    /// View a user’s email address.
    ///
    /// **API**
    ///
    /// - [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) (optional)
    /// - [Update User](https://dev.twitch.tv/docs/api/reference/#update-user) (optional)
    ///
    /// **EventSub**
    ///
    /// - [User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userupdate) (optional)
    UserReadEmail => "user:read:email",
    /// View emotes available to a user
    ///
    /// **API**
    ///
    /// - [Get User Emotes](https://dev.twitch.tv/docs/api/reference/#get-user-emotes)
    UserReadEmotes => "user:read:emotes",
    /// View the list of channels a user follows.
    ///
    /// **API**
    ///
    /// - [Get Followed Channels](https://dev.twitch.tv/docs/api/reference#get-followed-channels)
    /// - [Get Followed Streams](https://dev.twitch.tv/docs/api/reference#get-followed-streams)
    UserReadFollows => "user:read:follows",
    /// Read the list of channels you have moderator privileges in.
    ///
    /// **API**
    ///
    /// - [Get Moderated Channels](https://dev.twitch.tv/docs/api/reference#get-moderated-channels)
    UserReadModeratedChannels => "user:read:moderated_channels",
    /// View if an authorized user is subscribed to specific channels.
    ///
    /// **API**
    ///
    /// - [Check User Subscription](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
    UserReadSubscriptions => "user:read:subscriptions",
    /// Receive whispers sent to your user.
    ///
    /// **EventSub**
    ///
    /// - [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage)
    UserReadWhispers => "user:read:whispers",
    /// Receive whispers sent to your user, and send whispers on your user’s behalf.
    ///
    /// **API**
    ///
    /// - [Send Whisper](https://dev.twitch.tv/docs/api/reference#send-whisper)
    ///
    /// **EventSub**
    ///
    /// - [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage)
    UserManageWhispers => "user:manage:whispers",
    /// Send chat messages to a chatroom.
    ///
    /// **API**
    ///
    /// - [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)
    UserWriteChat => "user:write:chat",

    /// Send chat messages to a chatroom using an IRC connection.
    ChatEdit => "chat:edit",
    /// View chat messages sent in a chatroom using an IRC connection.
    ChatRead => "chat:read",
    /// Receive whisper messages for your user using PubSub.
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
