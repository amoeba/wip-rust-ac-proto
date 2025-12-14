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

// Performs a non autonomous jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump_NonAutonomous")]
pub struct MovementJumpNonAutonomous {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

impl crate::readers::ACDataType for MovementJumpNonAutonomous {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

