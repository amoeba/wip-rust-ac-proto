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

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_GetInscriptionResponse")]
pub struct ItemGetInscriptionResponse {
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
    #[serde(rename = "ScribeAccount")]
    pub scribe_account: String,
}

