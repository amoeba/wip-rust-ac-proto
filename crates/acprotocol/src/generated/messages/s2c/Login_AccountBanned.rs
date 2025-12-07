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

// Account has been banned
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBanned")]
pub struct LoginAccountBanned {
    #[serde(rename = "BannedUntil")]
    pub banned_until: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

