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

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyGlobalSquelch")]
pub struct CommunicationModifyGlobalSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationModifyGlobalSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            add,
            type_,
        })
    }
}

