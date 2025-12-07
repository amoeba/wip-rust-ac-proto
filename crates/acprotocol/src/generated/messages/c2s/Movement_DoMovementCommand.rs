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

// Performs a movement based on input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_DoMovementCommand")]
pub struct MovementDoMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

