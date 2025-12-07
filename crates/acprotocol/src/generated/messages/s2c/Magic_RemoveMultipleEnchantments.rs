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

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveMultipleEnchantments")]
pub struct MagicRemoveMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

