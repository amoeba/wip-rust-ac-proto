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

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ChangeStoragePermission")]
pub struct HouseChangeStoragePermission {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
    #[serde(rename = "HasPermission")]
    pub has_permission: bool,
}

