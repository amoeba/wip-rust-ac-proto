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

// Splits an item to a wield location.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToWield")]
pub struct InventoryStackableSplitToWield {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
    #[serde(rename = "Amount")]
    pub amount: i32,
}

impl crate::readers::ACDataType for InventoryStackableSplitToWield {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        let amount = read_i32(reader)?;

        Ok(Self {
            object_id,
            slot,
            amount,
        })
    }
}

