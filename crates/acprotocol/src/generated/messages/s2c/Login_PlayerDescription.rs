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

// Information describing your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_PlayerDescription")]
pub struct LoginPlayerDescription {
    #[serde(rename = "BaseQualities")]
    pub base_qualities: ACBaseQualities,
    #[serde(rename = "Qualities")]
    pub qualities: ACQualities,
    #[serde(rename = "PlayerModule")]
    pub player_module: PlayerModule,
    #[serde(rename = "ContentProfile")]
    pub content_profile: PackableList<ContentProfile>,
    #[serde(rename = "InventoryPlacement")]
    pub inventory_placement: PackableList<InventoryPlacement>,
}

