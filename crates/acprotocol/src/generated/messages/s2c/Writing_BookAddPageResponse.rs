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

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPageResponse")]
pub struct WritingBookAddPageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

