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

// Character creation screen initilised.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterCharGenVerificationResponseType1 {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SecondsUntilDeletion")]
    pub seconds_until_deletion: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharGenVerificationResponse")]
#[serde(tag = "ResponseType")]
pub enum CharacterCharGenVerificationResponse {
    #[serde(rename = "0x01")]
    Type1(CharacterCharGenVerificationResponseType1),
}

