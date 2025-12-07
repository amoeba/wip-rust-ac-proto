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

// Spend skill credits to train a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkillAdvancementClass")]
pub struct TrainTrainSkillAdvancementClass {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Credits")]
    pub credits: u32,
}

