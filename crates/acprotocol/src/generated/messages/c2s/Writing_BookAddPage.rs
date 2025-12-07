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

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPage")]
pub struct WritingBookAddPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

