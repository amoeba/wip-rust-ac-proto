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

// Makes a chess move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Move")]
pub struct GameMove {
    #[serde(rename = "XFrom")]
    pub x_from: i32,
    #[serde(rename = "YFrom")]
    pub y_from: i32,
    #[serde(rename = "XTo")]
    pub x_to: i32,
    #[serde(rename = "YTo")]
    pub y_to: i32,
}

