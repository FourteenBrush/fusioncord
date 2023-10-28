use crate::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadCreate(pub Channel);

impl Deref for ThreadCreate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThreadCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use serde::de::DeserializeSeed;
    use serde_json::Deserializer;

    use crate::{
        channel::{
            thread::{AutoArchiveDuration, ThreadMetadata},
            Channel, ChannelFlags, ChannelType,
        },
        gateway::{
            event::{DispatchEvent, GatewayEvent, GatewayEventDeserializer},
            payload::incoming::ThreadCreate,
        },
        id::Id,
        util::Timestamp,
    };

    #[test]
    fn simple_deserialisation() {
        let json = r#"{
            "t": "THREAD_CREATE",
            "s": 7,
            "op": 0,
            "d": {
                "type": 11,
                "total_message_sent": 0,
                "thread_metadata": {
                    "locked": false,
                    "create_timestamp": "2023-10-14T14:21:59.176000+00:00",
                    "auto_archive_duration": 1440,
                    "archived": false,
                    "archive_timestamp": "2023-10-14T14:21:59.176000+00:00"
                },
                "rate_limit_per_user": 0,
                "parent_id": "1023632039829831811",
                "owner_id": "638397185582563378",
                "newly_created": true,
                "name": "UML Diagram referencing",
                "message_count": 0,
                "member_ids_preview": [
                    "638397185582563378"
                ],
                "member_count": 1,
                "last_message_id": null,
                "id": "1162757198791835820",
                "guild_id": "648956210850299986",
                "flags": 0
            }
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(json).unwrap();
        let event = deserializer
            .deserialize(&mut Deserializer::from_str(json))
            .unwrap();
        if let GatewayEvent::Dispatch(_, DispatchEvent::ThreadCreate(tc)) = event {
            assert_eq!(
                *tc,
                ThreadCreate(Channel {
                    kind: ChannelType::PublicThread,
                    thread_metadata: Some(ThreadMetadata {
                        locked: false,
                        create_timestamp: Some(
                            Timestamp::parse("2023-10-14T14:21:59.176000+00:00").unwrap()
                        ),
                        auto_archive_duration: AutoArchiveDuration::Day,
                        archived: false,
                        archive_timestamp: Timestamp::parse("2023-10-14T14:21:59.176000+00:00")
                            .unwrap(),
                        invitable: None,
                    }),
                    rate_limit_per_user: Some(0),
                    parent_id: Some(Id::new(1023632039829831811)),
                    owner_id: Some(Id::new(638397185582563378)),
                    newly_created: Some(true),
                    name: Some("UML Diagram referencing".to_owned()),
                    message_count: Some(0),
                    member_count: Some(1),
                    last_message_id: None,
                    id: Id::new(1162757198791835820),
                    guild_id: Some(Id::new(648956210850299986)),
                    flags: Some(ChannelFlags::empty()),
                    position: None,
                    permission_overwrites: None,
                    topic: None,
                    nsfw: None,
                    user_limit: None,
                    recipients: None,
                    icon: None,
                    application_id: None,
                    managed: None,
                    last_pin_timestamp: None,
                    rtc_region: None,
                    video_quality_mode: None,
                    member: None,
                    default_auto_archive_duration: None,
                    available_tags: None,
                    applied_tags: None,
                    default_reaction_emoji: None,
                    default_thread_rate_limit_per_user: None,
                    default_sort_order: None,
                    default_forum_layout: None,
                    invitable: None,
                    bitrate: None,
                })
            )
        }
    }
}
