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

// Sets a new fill complevel for a component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetDesiredComponentLevel")]
pub struct CharacterSetDesiredComponentLevel {
    #[serde(rename = "Wcid")]
    pub wcid: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

