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

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleDefenderNotificationEvent")]
pub struct CombatHandleDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
    #[serde(rename = "Type")]
    pub type_: DamageType,
    #[serde(rename = "DamagePercent")]
    pub damage_percent: f32,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Location")]
    pub location: DamageLocation,
    #[serde(rename = "Critical")]
    pub critical: bool,
    #[serde(rename = "AttackConditions")]
    pub attack_conditions: AttackConditionsMask,
}

