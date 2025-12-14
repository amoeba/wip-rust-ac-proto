use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

// Sends an autonomous position
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomousPosition")]
pub struct MovementAutonomousPosition {
    #[serde(rename = "Position")]
    pub position: AutonomousPositionPack,
}

impl crate::readers::ACDataType for MovementAutonomousPosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let position = AutonomousPositionPack::read(reader)?;

        Ok(Self {
            position,
        })
    }
}

