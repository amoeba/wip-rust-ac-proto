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

// Direct message by name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirectByName")]
pub struct CommunicationTalkDirectByName {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl crate::readers::ACDataType for CommunicationTalkDirectByName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let target_name = read_string(reader)?;

        Ok(Self {
            message,
            target_name,
        })
    }
}

