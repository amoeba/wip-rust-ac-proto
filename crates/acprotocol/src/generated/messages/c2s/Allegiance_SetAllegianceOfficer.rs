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

// Sets an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficer")]
pub struct AllegianceSetAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
}

