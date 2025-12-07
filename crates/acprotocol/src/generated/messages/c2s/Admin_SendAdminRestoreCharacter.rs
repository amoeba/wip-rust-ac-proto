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

// Admin command to restore a character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_SendAdminRestoreCharacter")]
pub struct AdminSendAdminRestoreCharacter {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "RestoredCharName")]
    pub restored_char_name: String,
    #[serde(rename = "AccountToRestoreTo")]
    pub account_to_restore_to: String,
}

