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

// Spend XP to raise a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkill")]
pub struct TrainTrainSkill {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

