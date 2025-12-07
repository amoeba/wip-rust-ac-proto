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

// Gags a person in allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatGag")]
pub struct AllegianceAllegianceChatGag {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "On")]
    pub on: bool,
}

