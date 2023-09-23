use crate::gateway::presence::{Activity, ClientStatus, Status, UserOrId};
use serde::{Deserialize, Serialize};

/// User's presence was updated.
///
/// This may be received when a user's activity, status, or user
/// information - such as avatar or username - is updated.
///
/// Requires the [`Intents::GUILD_PRESENCES`] intent to receive this event.
///
/// Refer to [Discord Docs/Presence Update] for additional information.
///
/// [`Intents::GUILD_PRESENCES`]: crate::gateway::Intents
/// [Discord Docs/Presence Update]: https://discord.com/developers/docs/topics/gateway#presence-update
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PresenceUpdate {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub status: Status,
    pub user: UserOrId,
}

#[cfg(test)]
mod tests {
    use super::PresenceUpdate;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        PresenceUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
}
