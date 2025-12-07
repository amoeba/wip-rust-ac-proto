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

// Boots a player from the allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatBoot")]
pub struct AllegianceAllegianceChatBoot {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Reason")]
    pub reason: String,
}

