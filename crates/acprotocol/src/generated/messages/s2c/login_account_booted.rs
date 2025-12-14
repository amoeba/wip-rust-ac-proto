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

// Account has been booted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBooted")]
pub struct LoginAccountBooted {
    #[serde(rename = "AdditionalReasonText")]
    pub additional_reason_text: String,
    #[serde(rename = "ReasonText")]
    pub reason_text: String,
}

impl crate::readers::ACDataType for LoginAccountBooted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let additional_reason_text = read_string(reader)?;
        let reason_text = read_string(reader)?;

        Ok(Self {
            additional_reason_text,
            reason_text,
        })
    }
}

