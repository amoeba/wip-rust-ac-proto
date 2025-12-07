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

// Updates a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookModifyPage")]
pub struct WritingBookModifyPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
    #[serde(rename = "PageText")]
    pub page_text: String,
}

