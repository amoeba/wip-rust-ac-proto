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

// The name of the current world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_WorldInfo")]
pub struct LoginWorldInfo {
    #[serde(rename = "Connections")]
    pub connections: u32,
    #[serde(rename = "MaxConnections")]
    pub max_connections: u32,
    #[serde(rename = "WorldName")]
    pub world_name: String,
}

