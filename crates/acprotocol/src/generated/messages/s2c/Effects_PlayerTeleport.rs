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

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayerTeleport")]
pub struct EffectsPlayerTeleport {
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
}

