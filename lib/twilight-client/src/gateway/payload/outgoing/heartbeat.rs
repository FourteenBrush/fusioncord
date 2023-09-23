use crate::gateway::GatewayOpcode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Heartbeat {
    pub d: Option<u64>,
    pub op: GatewayOpcode,
}

impl Heartbeat {
    pub const fn new(seq: Option<u64>) -> Self {
        Self {
            d: seq,
            op: GatewayOpcode::Heartbeat,
        }
    }
}
