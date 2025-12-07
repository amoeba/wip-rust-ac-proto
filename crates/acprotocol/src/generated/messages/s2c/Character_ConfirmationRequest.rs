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

// Display a confirmation panel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationRequest")]
pub struct CharacterConfirmationRequest {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

