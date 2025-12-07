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

// Set a single character option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_PlayerOptionChangedEvent")]
pub struct CharacterPlayerOptionChangedEvent {
    #[serde(rename = "Option")]
    pub option: CharacterOptions1,
    #[serde(rename = "Value")]
    pub value: bool,
}

