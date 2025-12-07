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

// Contains the text associated with an emote action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSoulEmote")]
pub struct CommunicationHearSoulEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

