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

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute")]
pub struct QualitiesPrivateUpdateAttribute {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: AttributeInfo,
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = AttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

