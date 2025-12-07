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

// Admin Receive Player Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceivePlayerData")]
pub struct AdminReceivePlayerData {
    #[serde(rename = "Unknown")]
    pub unknown: i32,
    #[serde(rename = "AdminPlayerData")]
    pub admin_player_data: PackableList<AdminPlayerData>,
}

