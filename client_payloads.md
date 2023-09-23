# CHANNEL_UNREAD_UPDATE

*The payload appears to provide a snapshot of unread message updates for multiple channels within a Discord server or guild, but without more context, it's challenging to interpret the specific changes or events that triggered this update. To understand the meaning of this payload fully, you would need to refer to the Discord API documentation and potentially analyze the broader context of the application using it.

Here are some possible scenarios in which the CHANNEL_UNREAD_UPDATE payload may be sent:

    - When a user receives a new message in a channel and the unread message count increases.
    - When a user reads all the messages in a channel and the unread message count becomes zero.
    - When a user marks a specific message or a range of messages as read, causing the unread message count to decrease.

*

- channel_unread_updates: Array of unread update:
    - id: Snowflake in String format
    - last_message_id: presumably a Snowflake in String format
    - last_pin_timestamp: timestamp with timezone (got "2021-02-17T19:29:53.999000+00:00")
- guild_id: Snowflake

# MESSAGE_ACK

- channel_id: Snowflake
- flags: bitset maybe?
- last_viewed: presumably an integer (got 3148)
- message_id: presumably a Snowflake in String format
- version: integer

# SESSIONS_REPLACE

payload is an array with session objects:
- activities: Array of activity
- client_info: ClientInfo:
    - client: String,
    - os: String,
    - version: integer
- session_id: String
- status: String

# USER_GUILD_SETTINGS_UPDATE

- channel_overrides: Array of overrides:
    - channel_id: presumably a Snowflake in String format
    - collapsed: bool
    - flags: integer
    - message_notifications: integer
    - mute_config: unknown (got null)
    - muted: bool
- flags: integer
- guild_id: presumably a Snowflake in String format
- hide_muted_channels: bool
- message_notifications: integer
- mobile_push: bool
- mute_config: unknown (got null)
- mute_scheduled_events: bool
- muted: bool
- notifify_highlights: integer
- suppress_everyone: bool
- suppress_roles: bool
- version: integer

# USER_SETTINGS_PROTO_UPDATE

- partial: bool
- settings: Settings object:
    - proto: String
    - type: integer

# USER_SETTINGS_UPDATE

- message_display_compact: bool (optional)
- theme: String (optional)

# RELATIONSHIP_REMOVE

- id: Snowflake
- nickname: unknown (got null, presumably String)
- type: integer

# RELATIONSHIP_ADD

- id: Snowflake in String format
- nickname: unknown (got null)
- should_notify: bool
- since: timestamp with timezone
- type: integer
- user: User object:
    - avatar: String
    - avatar_decoration: presumably String (got null)
    - discriminator: String
    - global_name: String
    - id: String
    - public_flags: integer
    - username: String

# BURST_CREDIT_BALANCE_UPDATE

- amount: integer
- next_replenish_at: timestamp with timezone
- replenished_today: bool

# NOTIFICATION_CENTER_ITEM_CREATE

- acked: bool
- body: String
- bundle_id: String
- completed: bool
- deeplink: String
- disable_action: bool
- guild_id: unknown (got null)
- guild_scheduled_event_id: unknown (got null)
- icon_name: unknown (got null)
- icon_url: String
- id: Snowflake in String format
- is_voice_message: bool
- item_enum: unknown (got null)
- message: unknown (got null)
- message_attachment_count: unknown (got null)
- message_channel_id: unknown (got null)
- message_content: unknown (got null)
- message_embed_count: unknown (got null)
- message_id: unknown (got null)
- message_sticker_count: unknown (got null)
- other_user: User object or something:
    - avatar: String
    - avatar_decoration: unknown (got null)
    - discriminator: String
    - global_name: String
    - id: Snowflake as String
    - public_flags: integer
    - username: String
- type: String

# NOTIFICATION_CENTER_ITEM_COMPLETED

- item_enum: integer