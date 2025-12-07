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

// The list of characters on the current account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LoginCharacterSet")]
pub struct LoginLoginCharacterSet {
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Characters")]
    pub characters: PackableList<CharacterIdentity>,
    #[serde(rename = "DeletedCharacters")]
    pub deleted_characters: PackableList<CharacterIdentity>,
    #[serde(rename = "NumAllowedCharacters")]
    pub num_allowed_characters: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "UseTurbineChat")]
    pub use_turbine_chat: bool,
    #[serde(rename = "HasThroneofDestiny")]
    pub has_throneof_destiny: bool,
}

