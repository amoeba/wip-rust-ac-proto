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

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelMultipleEnchantments")]
pub struct MagicDispelMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

