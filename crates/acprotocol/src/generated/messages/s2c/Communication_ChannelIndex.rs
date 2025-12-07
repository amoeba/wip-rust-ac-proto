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

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {
    #[serde(rename = "Channels")]
    pub channels: PackableList<string>,
}

