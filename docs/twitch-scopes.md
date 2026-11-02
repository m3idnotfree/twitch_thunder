---
checked: 2026-11-04
source: https://dev.twitch.tv/docs/authentication/scopes/
---

## Twitch API and EventSub scopes

### `analytics:read:extensions`

View analytics data for the Twitch Extensions owned by the authenticated account.

**API**

- [Get Extension Analytics](https://dev.twitch.tv/docs/api/reference#get-extension-analytics)

### `analytics:read:games`

View analytics data for the games owned by the authenticated account.

**API**

- [Get Game Analytics](https://dev.twitch.tv/docs/api/reference#get-game-analytics)

### `bits:read`

View Bits-related products and redemptions for a channel.

**API**

- [Get Bits Leaderboard](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
- [Get Custom Power-up](https://dev.twitch.tv/docs/api/reference/#get-custom-power-up)

**EventSub**

- [Channel Bits Use](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelbitsuse)
- [Channel Cheer](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer)
- [Channel Custom Power-ups Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcustom_power_up_redemptionadd)

### `channel:bot`

Joins your channel’s chatroom as a bot user, and perform chat-related actions as that user.

**API**

- [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)

**EventSub**

- [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
- [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
- [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
- [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
- [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
- [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)

### `channel:manage:ads`

Manage ads schedule on a channel.

**API**

- [Snooze Next Ad](https://dev.twitch.tv/docs/api/reference#snooze-next-ad)

### `channel:read:ads`

Read the ads schedule and details on your channel.

**API**

- [Get Ad Schedule](https://dev.twitch.tv/docs/api/reference#get-ad-schedule)

**EventSub**

- [Channel Ad Break Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin)

### `channel:manage:broadcast`

Manage a channel’s broadcast configuration, including updating channel configuration and managing stream markers and stream tags.

**API**

- [Modify Channel Information](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
- [Create Stream Marker](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
- [Replace Stream Tags](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)

### `channel:read:charity`

Read charity campaign details and user donations on your channel.

**API**

- [Get Charity Campaign](https://dev.twitch.tv/docs/api/reference#get-charity-campaign)
- [Get Charity Campaign Donations](https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations)

**EventSub**

- [Charity Donation](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaigndonate)
- [Charity Campaign Start](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstart)
- [Charity Campaign Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignprogress)
- [Charity Campaign Stop](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstop)

### `channel:manage:clips`

Manage Clips for a channel.

**API**

- [Create Clip From VOD](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod)
- [Get Clips Download](https://dev.twitch.tv/docs/api/reference#get-clips-download)

### `channel:edit:commercial`

Run commercials on a channel.

**API**

- [Start Commercial](https://dev.twitch.tv/docs/api/reference#start-commercial)

### `channel:read:editors`

View a list of users with the editor role for a channel.

**API**

- [Get Channel Editors](https://dev.twitch.tv/docs/api/reference#get-channel-editors)

### `channel:manage:extensions`

Manage a channel’s Extension configuration, including activating Extensions.

**API**

- [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
- [Update User Extensions](https://dev.twitch.tv/docs/api/reference#update-user-extensions)

### `channel:read:goals`

View Creator Goals for a channel.

**API**

- [Get Creator Goals](https://dev.twitch.tv/docs/api/reference#get-creator-goals)

**EventSub**

- [Goal Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalbegin)
- [Goal Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalprogress)
- [Goal End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalend)

### `channel:read:guest_star`

Read Guest Star details for your channel.

**API**

- [Get Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#get-channel-guest-star-settings)
- [Get Guest Star Session](https://dev.twitch.tv/docs/api/reference#get-guest-star-session)
- [Get Guest Star Invites](https://dev.twitch.tv/docs/api/reference#get-guest-star-invites)

**EventSub**

- [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
- [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
- [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
- [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)

### `channel:manage:guest_star`

Manage Guest Star for your channel.

**API**

- [Update Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#update-channel-guest-star-settings)
- [Create Guest Star Session](https://dev.twitch.tv/docs/api/reference#create-guest-star-session)
- [End Guest Star Session](https://dev.twitch.tv/docs/api/reference#end-guest-star-session)
- [Send Guest Star Invite](https://dev.twitch.tv/docs/api/reference#send-guest-star-invite)
- [Delete Guest Star Invite](https://dev.twitch.tv/docs/api/reference#delete-guest-star-invite)
- [Assign Guest Star Slot](https://dev.twitch.tv/docs/api/reference#assign-guest-star-slot)
- [Update Guest Star Slot](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot)
- [Delete Guest Star Slot](https://dev.twitch.tv/docs/api/reference#delete-guest-star-slot)
- [Update Guest Star Slot Settings](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot-settings)

**EventSub**

- [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
- [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
- [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
- [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)

### `channel:read:hype_train`

View Hype Train information for a channel.

**API**

- [Get Hype Train Status](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)

**EventSub**

- [Hype Train Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainbegin)
- [Hype Train Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainprogress)
- [Hype Train End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainend)

### `channel:manage:moderators`

Add or remove the moderator role from users in your channel.

**API**

- [Add Channel Moderator](https://dev.twitch.tv/docs/api/reference#add-channel-moderator)
- [Remove Channel Moderator](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator)
- [Get Moderators](https://dev.twitch.tv/docs/api/reference/#get-moderators)

### `channel:read:polls`

View a channel’s polls.

**API**

- [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls)

**EventSub**

- [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)
- [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)
- [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)

### `channel:manage:polls`

Manage a channel’s polls.

**API**

- [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls)
- [Create Poll](https://dev.twitch.tv/docs/api/reference#create-poll)
- [End Poll](https://dev.twitch.tv/docs/api/reference#end-poll)

**EventSub**

- [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)
- [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)
- [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)

### `channel:read:predictions`

View a channel’s Channel Points Predictions.

**API**

- [Get Channel Points Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions)

**EventSub**

- [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)
- [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)
- [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)
- [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)

### `channel:manage:predictions`

Manage of channel’s Channel Points Predictions

**API**

- [Get Channel Points Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions)
- [Create Channel Points Prediction](https://dev.twitch.tv/docs/api/reference#create-prediction)
- [End Channel Points Prediction](https://dev.twitch.tv/docs/api/reference#end-prediction)

**EventSub**

- [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)
- [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)
- [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)
- [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)

### `channel:manage:raids`

Manage a channel raiding another channel.

**API**

- [Start a raid](https://dev.twitch.tv/docs/api/reference#start-a-raid)
- [Cancel a raid](https://dev.twitch.tv/docs/api/reference#cancel-a-raid)

### `channel:read:redemptions`

View Channel Points custom rewards and their redemptions on a channel.

**API**

- [Get Custom Reward](https://dev.twitch.tv/docs/api/reference#get-custom-reward)
- [Get Custom Reward Redemption](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)

**EventSub**

- [Channel Points Automatic Reward Redemption](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)
- [Channel Points Automatic Reward Redemption v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd-v2)
- [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)
- [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)
- [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)
- [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)
- [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)

### `channel:manage:redemptions`

Manage Channel Points custom rewards and their redemptions on a channel.

**API**

- [Get Custom Reward](https://dev.twitch.tv/docs/api/reference#get-custom-reward)
- [Get Custom Reward Redemption](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
- [Create Custom Rewards](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
- [Delete Custom Reward](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
- [Update Custom Reward](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
- [Update Redemption Status](https://dev.twitch.tv/docs/api/reference#update-redemption-status)

**EventSub**

- [Channel Points Automatic Reward Redemption](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)
- [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)
- [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)
- [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)
- [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)
- [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)

### `channel:manage:schedule`

Manage a channel’s stream schedule.

**API**

- [Update Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule)
- [Create Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
- [Update Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
- [Delete Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)

### `channel:read:stream_key`

View an authorized user’s stream key.

**API**

- [Get Stream Key](https://dev.twitch.tv/docs/api/reference#get-stream-key)

### `channel:read:subscriptions`

View a list of all subscribers to a channel and check if a user is subscribed to a channel.

**API**

- [Get Broadcaster Subscriptions](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)

**EventSub**

- [Channel Subscribe](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscribe)
- [Channel Subscription End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionend)
- [Channel Subscription Gift](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptiongift)
- [Channel Subscription Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionmessage)

### `channel:manage:videos`

Manage a channel’s videos, including deleting videos.

**API**

- [Delete Videos](https://dev.twitch.tv/docs/api/reference#delete-videos)

### `channel:read:vips`

Read the list of VIPs in your channel.

**API**

- [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips)

**EventSub**

- [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)
- [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)

### `channel:manage:vips`

Add or remove the VIP role from users in your channel.

**API**

- [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips)
- [Add Channel VIP](https://dev.twitch.tv/docs/api/reference#add-channel-vip)
- [Remove Channel VIP](https://dev.twitch.tv/docs/api/reference#remove-channel-vip)

**EventSub**

- [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)
- [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)

### `channel:moderate`

Perform moderation actions in a channel.

**EventSub**

- [Channel Ban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelban)
- [Channel Unban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban)

### `clips:edit`

Manage Clips for a channel.

**API**

- [Create Clip](https://dev.twitch.tv/docs/api/reference#create-clip)

### `editor:manage:clips`

Manage Clips as an editor.

**API**

- [Create Clip From VOD](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod)
- [Get Clips Download](https://dev.twitch.tv/docs/api/reference#get-clips-download)

### `moderation:read`

View a channel’s moderation data including Moderators, Bans, Timeouts, and Automod settings.

**API**

- [Check AutoMod Status](https://dev.twitch.tv/docs/api/reference#check-automod-status)
- [Get Banned Users](https://dev.twitch.tv/docs/api/reference#get-banned-users)
- [Get Moderators](https://dev.twitch.tv/docs/api/reference#get-moderators)

**EventSub**

- [Channel Moderator Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatoradd)
- [Channel Moderator Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatorremove)

### `moderator:manage:announcements`

Send announcements in channels where you have the moderator role.

**API**

- [Send Chat Announcement](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)

### `moderator:manage:automod`

Manage messages held for review by AutoMod in channels where you are a moderator.

**API**

- [Manage Held AutoMod Messages](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)

**EventSub**

- [AutoMod Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold)
- [AutoMod Message Hold v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold-v2)
- [AutoMod Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate)
- [AutoMod Message Update v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate-v2)
- [AutoMod Terms Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodtermsupdate)

### `moderator:read:automod_settings`

View a broadcaster’s AutoMod settings.

**API**

- [Get AutoMod Settings](https://dev.twitch.tv/docs/api/reference#get-automod-settings)

**EventSub**

- [AutoMod Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodsettingsupdate)

### `moderator:manage:automod_settings`

Manage a broadcaster’s AutoMod settings.

**API**

- [Update AutoMod Settings](https://dev.twitch.tv/docs/api/reference#update-automod-settings)

### `moderator:read:banned_users`

Read the list of bans or unbans in channels where you have the moderator role.

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)

### `moderator:manage:banned_users`

Ban and unban users.

**API**

- [Get Banned Users](https://dev.twitch.tv/docs/api/reference/#get-banned-users)
- [Ban User](https://dev.twitch.tv/docs/api/reference#ban-user)
- [Unban User](https://dev.twitch.tv/docs/api/reference#unban-user)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)

### `moderator:read:blocked_terms`

View a broadcaster’s list of blocked terms.

**API**

- [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:manage:blocked_terms`

Manage a broadcaster’s list of blocked terms.

**API**

- [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
- [Add Blocked Term](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
- [Remove Blocked Term](https://dev.twitch.tv/docs/api/reference#remove-blocked-term)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:read:chat_messages`

Read deleted chat messages in channels where you have the moderator role and get pinned chat messages.

**API**

- [Get Pinned Chat Message](https://dev.twitch.tv/docs/api/reference#get-pinned-chat-message)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:manage:chat_messages`

Delete chat messages in channels where you have the moderator role and manage pinned chat messages.

**API**

- [Delete Chat Messages](https://dev.twitch.tv/docs/api/reference#delete-chat-messages)
- [Pin Chat Message](https://dev.twitch.tv/docs/api/reference#pin-chat-message)
- [Update Pinned Chat Message](https://dev.twitch.tv/docs/api/reference#update-pinned-chat-message)
- [Unpin Chat Message](https://dev.twitch.tv/docs/api/reference#unpin-chat-message)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:read:chat_settings`

View a broadcaster’s chat room settings.

**API**

- [Get Chat Settings](https://dev.twitch.tv/docs/api/reference#get-chat-settings)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:manage:chat_settings`

Manage a broadcaster’s chat room settings.

**API**

- [Update Chat Settings](https://dev.twitch.tv/docs/api/reference#update-chat-settings)

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:read:chatters`

View the chatters in a broadcaster’s chat room.

**API**

- [Get Chatters](https://dev.twitch.tv/docs/api/reference#get-chatters)

### `moderator:read:followers`

Read the followers of a broadcaster.

**API**

- [Get Channel Followers](https://dev.twitch.tv/docs/api/reference#get-channel-followers)

**EventSub**

- [Channel Follow](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow)

### `moderator:read:guest_star`

Read Guest Star details for channels where you are a Guest Star moderator.

**API**

- [Get Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#get-channel-guest-star-settings)
- [Get Guest Star Session](https://dev.twitch.tv/docs/api/reference#get-guest-star-session)
- [Get Guest Star Invites](https://dev.twitch.tv/docs/api/reference#get-guest-star-invites)

**EventSub**

- [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
- [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
- [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
- [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)

### `moderator:manage:guest_star`

Manage Guest Star for channels where you are a Guest Star moderator.

**API**

- [Send Guest Star Invite](https://dev.twitch.tv/docs/api/reference#send-guest-star-invite)
- [Delete Guest Star Invite](https://dev.twitch.tv/docs/api/reference#delete-guest-star-invite)
- [Assign Guest Star Slot](https://dev.twitch.tv/docs/api/reference#assign-guest-star-slot)
- [Update Guest Star Slot](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot)
- [Delete Guest Star Slot](https://dev.twitch.tv/docs/api/reference#delete-guest-star-slot)
- [Update Guest Star Slot Settings](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot-settings)

**EventSub**

- [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin)
- [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend)
- [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate)
- [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate)

### `moderator:read:moderators`

Read the list of moderators in channels where you have the moderator role.

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)

### `moderator:read:shield_mode`

View a broadcaster’s Shield Mode status.

**API**

- [Get Shield Mode Status](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status)

**EventSub**

- [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)
- [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)

### `moderator:manage:shield_mode`

Manage a broadcaster’s Shield Mode status.

**API**

- [Update Shield Mode Status](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status)

**EventSub**

- [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)
- [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)

### `moderator:read:shoutouts`

View a broadcaster’s shoutouts.

**EventSub**

- [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)
- [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)

### `moderator:manage:shoutouts`

Manage a broadcaster’s shoutouts.

**API**

- [Send a Shoutout](https://dev.twitch.tv/docs/api/reference#send-a-shoutout)

**EventSub**

- [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)
- [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)

### `moderator:read:suspicious_users`

Read chat messages from suspicious users and see users flagged as suspicious in channels where the user has the moderator role.

**EventSub**

- [Channel Suspicious User Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_usermessage)
- [Channel Suspicious User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_userupdate)

### `moderator:manage:suspicious_users`

Manage suspicious user statuses in channels where the user has the moderator role.

**API**

- [Add suspicious status to chat user](https://dev.twitch.tv/docs/api/reference/#add-suspicious-status-to-chat-user)
- [Remove suspicious status from chat user](https://dev.twitch.tv/docs/api/reference/#remove-suspicious-status-from-chat-user)

### `moderator:read:unban_requests`

View a broadcaster’s unban requests.

**API**

- [Get Unban Requests](https://dev.twitch.tv/docs/api/reference/#get-unban-requests)

**EventSub**

- [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)
- [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)
- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:manage:unban_requests`

Manage a broadcaster’s unban requests.

**API**

- [Resolve Unban Requests](https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests)

**EventSub**

- [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)
- [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)
- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

### `moderator:read:vips`

Read the list of VIPs in channels where you have the moderator role.

**EventSub**

- [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)

### `moderator:read:warnings`

Read warnings in channels where you have the moderator role.

**EventSub**

- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
- [Channel Warning Acknowledge](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)
- [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend)

### `moderator:manage:warnings`

Warn users in channels where you have the moderator role.

**API**

- [Warn Chat User](https://dev.twitch.tv/docs/api/reference#warn-chat-user)

**EventSub**

- [Channel Moderate v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
- [Channel Warning Acknowledge](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)
- [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend)

### `user:bot`

Join a specified chat channel as your user and appear as a bot, and perform chat-related actions as your user.

**API**

- [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)

**EventSub**

- [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
- [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
- [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
- [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
- [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
- [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
- [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_user_message_hold)
- [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)

### `user:edit`

Manage a user object.

**API**

- [Update User](https://dev.twitch.tv/docs/api/reference#update-user)

### `user:edit:broadcast`

View and edit a user’s broadcasting configuration, including Extension configurations.

**API**

- [Get User Extensions](https://dev.twitch.tv/docs/api/reference/#get-user-extensions)
- [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions)
- [Update User Extensions](https://dev.twitch.tv/docs/api/reference/#update-user-extensions)

### `user:read:blocked_users`

View the block list of a user.

**API**

- [Get User Block List](https://dev.twitch.tv/docs/api/reference#get-user-block-list)

### `user:manage:blocked_users`

Manage the block list of a user.

**API**

- [Block User](https://dev.twitch.tv/docs/api/reference#block-user)
- [Unblock User](https://dev.twitch.tv/docs/api/reference#unblock-user)

### `user:read:broadcast`

View a user’s broadcasting configuration, including Extension configurations.

**API**

- [Get Stream Markers](https://dev.twitch.tv/docs/api/reference#get-stream-markers)
- [Get User Extensions](https://dev.twitch.tv/docs/api/reference#get-user-extensions)
- [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)

### `user:read:chat`

Receive chatroom messages and informational notifications relating to a channel’s chatroom.

**EventSub**

- [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
- [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
- [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
- [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
- [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
- [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
- [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_hold)
- [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)

### `user:manage:chat_color`

Update the color used for the user’s name in chat.

**API**

- [Update User Chat Color](https://dev.twitch.tv/docs/api/reference#update-user-chat-color)

### `user:read:email`

View a user’s email address.

**API**

- [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) (optional)
- [Update User](https://dev.twitch.tv/docs/api/reference/#update-user) (optional)

**EventSub**

- [User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userupdate) (optional)

### `user:read:emotes`

View emotes available to a user

**API**

- [Get User Emotes](https://dev.twitch.tv/docs/api/reference/#get-user-emotes)

### `user:read:follows`

View the list of channels a user follows.

**API**

- [Get Followed Channels](https://dev.twitch.tv/docs/api/reference#get-followed-channels)
- [Get Followed Streams](https://dev.twitch.tv/docs/api/reference#get-followed-streams)

### `user:read:moderated_channels`

Read the list of channels you have moderator privileges in.

**API**

- [Get Moderated Channels](https://dev.twitch.tv/docs/api/reference#get-moderated-channels)

### `user:read:subscriptions`

View if an authorized user is subscribed to specific channels.

**API**

- [Check User Subscription](https://dev.twitch.tv/docs/api/reference#check-user-subscription)

### `user:read:whispers`

Receive whispers sent to your user.

**EventSub**

- [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage)

### `user:manage:whispers`

Receive whispers sent to your user, and send whispers on your user’s behalf.

**API**

- [Send Whisper](https://dev.twitch.tv/docs/api/reference#send-whisper)

**EventSub**

- [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage)

### `user:write:chat`

Send chat messages to a chatroom.

**API**

- [Send Chat Message](https://dev.twitch.tv/docs/api/reference/#send-chat-message)

## IRC Chat Scopes

### `chat:edit`

Send chat messages to a chatroom using an IRC connection.

### `chat:read`

View chat messages sent in a chatroom using an IRC connection.

## PubSub-specific Chat Scopes

### `whispers:read`

Receive whisper messages for your user using PubSub.
