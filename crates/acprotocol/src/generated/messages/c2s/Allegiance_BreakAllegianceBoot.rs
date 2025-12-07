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

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegianceBoot")]
pub struct AllegianceBreakAllegianceBoot {
    #[serde(rename = "BooteeName")]
    pub bootee_name: String,
    #[serde(rename = "AccountBoot")]
    pub account_boot: bool,
}

