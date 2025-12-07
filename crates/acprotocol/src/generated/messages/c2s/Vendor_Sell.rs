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

// Sell to a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_Sell")]
pub struct VendorSell {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
}

