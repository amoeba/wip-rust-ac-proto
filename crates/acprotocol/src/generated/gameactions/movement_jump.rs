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

// Performs a jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump")]
pub struct MovementJump {
    #[serde(rename = "Jump")]
    pub jump: JumpPack,
}

impl crate::readers::ACDataType for MovementJump {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let jump = JumpPack::read(reader)?;

        Ok(Self {
            jump,
        })
    }
}

