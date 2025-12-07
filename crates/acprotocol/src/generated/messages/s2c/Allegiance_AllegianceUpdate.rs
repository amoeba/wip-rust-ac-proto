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

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdate")]
pub struct AllegianceAllegianceUpdate {
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

