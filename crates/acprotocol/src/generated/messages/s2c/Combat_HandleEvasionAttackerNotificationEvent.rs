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

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
}

