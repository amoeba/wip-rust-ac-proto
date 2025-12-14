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

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearRangedSpeech")]
pub struct CommunicationHearRangedSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Range")]
    pub range: f32,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationHearRangedSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let sender_name = read_string(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let range = read_f32(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            message,
            sender_name,
            sender_id,
            range,
            type_,
        })
    }
}

