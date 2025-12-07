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

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackDoneEvent")]
pub struct CombatHandleAttackDoneEvent {
    #[serde(rename = "Number")]
    pub number: u32,
}

