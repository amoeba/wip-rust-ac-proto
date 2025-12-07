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

// The character to log in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_SendEnterWorld")]
pub struct LoginSendEnterWorld {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Account")]
    pub account: String,
}

