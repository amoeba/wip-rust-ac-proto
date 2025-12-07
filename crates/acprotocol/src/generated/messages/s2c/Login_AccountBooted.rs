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

// Account has been booted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBooted")]
pub struct LoginAccountBooted {
    #[serde(rename = "AdditionalReasonText")]
    pub additional_reason_text: String,
    #[serde(rename = "ReasonText")]
    pub reason_text: String,
}

