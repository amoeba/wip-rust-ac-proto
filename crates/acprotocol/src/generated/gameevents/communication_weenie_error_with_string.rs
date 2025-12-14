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

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieErrorWithString")]
pub struct CommunicationWeenieErrorWithString {
    #[serde(rename = "Type")]
    pub type_: WeenieErrorWithString,
    #[serde(rename = "Text")]
    pub text: String,
}

impl crate::readers::ACDataType for CommunicationWeenieErrorWithString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = WeenieErrorWithString::try_from(read_u32(reader)?)?;
        let text = read_string(reader)?;

        Ok(Self {
            type_,
            text,
        })
    }
}

