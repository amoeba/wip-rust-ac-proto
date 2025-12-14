use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

// Joins a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_AddToChannel")]
pub struct CommunicationAddToChannel {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl crate::readers::ACDataType for CommunicationAddToChannel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            channel,
        })
    }
}

