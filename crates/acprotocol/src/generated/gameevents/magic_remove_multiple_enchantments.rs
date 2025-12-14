use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveMultipleEnchantments")]
pub struct MagicRemoveMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

impl crate::readers::ACDataType for MagicRemoveMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

