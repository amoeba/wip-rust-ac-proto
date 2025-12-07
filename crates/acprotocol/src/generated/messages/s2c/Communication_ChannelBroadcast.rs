use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
    #[serde(rename = "Channel")]
    pub channel: Channel,
    #[serde(rename = "Message")]
    pub message: String,
}

