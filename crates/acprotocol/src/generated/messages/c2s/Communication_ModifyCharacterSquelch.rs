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

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyCharacterSquelch")]
pub struct CommunicationModifyCharacterSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

