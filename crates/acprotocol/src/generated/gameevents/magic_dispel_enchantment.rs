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

// Silently remove An enchantment from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelEnchantment")]
pub struct MagicDispelEnchantment {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl crate::readers::ACDataType for MagicDispelEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

