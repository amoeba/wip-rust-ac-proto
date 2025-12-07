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

// Direct message by name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirectByName")]
pub struct CommunicationTalkDirectByName {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

