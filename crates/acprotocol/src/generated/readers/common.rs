// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl WindowProperty_1000007F {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_j = read_u32(reader)?;
        let value_j = read_u64(reader)?;

        Ok(Self {
            unknown_j,
            value_j,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_1000007F {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_1000007F::read(reader)
    }
}

impl WindowProperty_10000086 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_i = read_u32(reader)?;
        let value_i = read_u32(reader)?;

        Ok(Self {
            unknown_i,
            value_i,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_10000086 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_10000086::read(reader)
    }
}

impl WindowProperty_10000087 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_h = read_u32(reader)?;
        let value_h = read_u32(reader)?;

        Ok(Self {
            unknown_h,
            value_h,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_10000087 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_10000087::read(reader)
    }
}

impl WindowProperty_10000088 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_f = read_u32(reader)?;
        let value_f = read_u32(reader)?;

        Ok(Self {
            unknown_f,
            value_f,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_10000088 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_10000088::read(reader)
    }
}

impl WindowProperty_10000089 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_e = read_u32(reader)?;
        let value_e = read_u32(reader)?;

        Ok(Self {
            unknown_e,
            value_e,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_10000089 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_10000089::read(reader)
    }
}

impl WindowProperty_1000008A {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_d = read_u32(reader)?;
        let value_d = read_u8(reader)?;

        Ok(Self {
            unknown_d,
            value_d,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_1000008A {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_1000008A::read(reader)
    }
}

impl WindowProperty_1000008D {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_c = read_u32(reader)?;
        let title_source = WindowProperty_1000008D_TitleSourceVariant::read(reader)?;

impl WindowProperty_1000008D_TitleSourceVariant {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let title_source = read_u8(reader)?;

        match title_source {
            0x00 => {
                let string_id = read_u32(reader)?;
                let file_id = read_u32(reader)?;
                Ok(Self::Type0 {
                    string_id,
                    file_id,
                }))
            },
            0x01 => {
                let value_a = WString::read(reader)?;
                Ok(Self::Type1 {
                    value_a,
                }))
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            unknown_c,
            title_source,
        })
    }
}

impl crate::readers::ACDataType for WindowProperty_1000008D {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty_1000008D::read(reader)
    }
}

