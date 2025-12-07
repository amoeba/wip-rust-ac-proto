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

// Update multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateMultipleEnchantments")]
pub struct MagicUpdateMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<Enchantment>,
}

