// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl PackedDWORD {
    pub fn read(_reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedDWORD {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        PackedDWORD::read(reader)
    }
}

impl ObjectId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for ObjectId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectId::read(reader)
    }
}

impl LandcellId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for LandcellId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        LandcellId::read(reader)
    }
}

impl Vector3 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Vector3 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Vector3::read(reader)
    }
}

impl Quaternion {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let w = read_f32(reader)?;
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            w,
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Quaternion {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Quaternion::read(reader)
    }
}

impl Position {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let frame = Frame::read(reader)?;

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl crate::readers::ACDataType for Position {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Position::read(reader)
    }
}

impl Frame {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Vector3::read(reader)?;
        let orientation = Quaternion::read(reader)?;

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl crate::readers::ACDataType for Frame {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Frame::read(reader)
    }
}

impl AllegianceRecord {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let tree_parent = ObjectId::read(reader)?;
        let allegiance_data = AllegianceData::read(reader)?;

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRecord {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRecord::read(reader)
    }
}

impl AllegianceHierarchy {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let record_count = read_u16(reader)?;
        let old_version = read_u16(reader)?;
        let officers = read_phash_table::<ObjectId, AllegianceOfficerLevel>(reader)?;
        let officer_titles = read_packable_list::<String>(reader)?;
        let monarch_broadcast_time = read_u32(reader)?;
        let monarch_broadcasts_today = read_u32(reader)?;
        let spokes_broadcast_time = read_u32(reader)?;
        let spokes_broadcasts_today = read_u32(reader)?;
        let motd = read_string(reader)?;
        let motd_set_by = read_string(reader)?;
        let chat_room_id = read_u32(reader)?;
        let bindpoint = Position::read(reader)?;
        let allegiance_name = read_string(reader)?;
        let name_last_set_time = read_u32(reader)?;
        let is_locked = read_bool(reader)?;
        let approved_vassal = read_i32(reader)?;
        let mut monarch_data = None;
        if record_count > 0 {
            monarch_data = Some(AllegianceData::read(reader)?);
        }
        let records = read_vec::<AllegianceRecord>(reader, (record_count as usize) - 1)?;

        Ok(Self {
            record_count,
            old_version,
            officers,
            officer_titles,
            monarch_broadcast_time,
            monarch_broadcasts_today,
            spokes_broadcast_time,
            spokes_broadcasts_today,
            motd,
            motd_set_by,
            chat_room_id,
            bindpoint,
            allegiance_name,
            name_last_set_time,
            is_locked,
            approved_vassal,
            monarch_data,
            records,
        })
    }
}

impl crate::readers::ACDataType for AllegianceHierarchy {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceHierarchy::read(reader)
    }
}

impl AllegianceData {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let xp_cached = read_u32(reader)?;
        let xp_tithed = read_u32(reader)?;
        let flags = read_u32(reader)?;
        let gender = Gender::try_from(read_u8(reader)?)?;
        let heritage = HeritageGroup::try_from(read_u8(reader)?)?;
        let rank = read_u16(reader)?;
        let mut level = None;
        if (flags & 0x8) != 0 {
            level = Some(read_u32(reader)?);
        }
        let loyalty = read_u16(reader)?;
        let leadership = read_u16(reader)?;
        let mut allegiance_age = None;
        let time_online;
        if flags == 0x4 {
            time_online = Some(read_u64(reader)?);
        } else {
            allegiance_age = Some(read_u32(reader)?);
            time_online = Some((read_u32(reader))? as u64);
        }
        let name = read_string(reader)?;

        Ok(Self {
            character_id,
            xp_cached,
            xp_tithed,
            flags,
            gender,
            heritage,
            rank,
            level,
            loyalty,
            leadership,
            allegiance_age,
            time_online,
            name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceData {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceData::read(reader)
    }
}

impl ObjDesc {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u8(reader)?;
        let palette_count = read_u8(reader)?;
        let texture_count = read_u8(reader)?;
        let model_count = read_u8(reader)?;
        let mut palette = None;
        if palette_count > 0 {
            palette = Some(DataId::read(reader)?);
        }
        let subpalettes = read_vec::<Subpalette>(reader, palette_count as usize)?;
        let tm_changes = read_vec::<TextureMapChange>(reader, texture_count as usize)?;
        let ap_changes = read_vec::<AnimPartChange>(reader, model_count as usize)?;

        Ok(Self {
            version,
            palette_count,
            texture_count,
            model_count,
            palette,
            subpalettes,
            tm_changes,
            ap_changes,
        })
    }
}

impl crate::readers::ACDataType for ObjDesc {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ObjDesc::read(reader)
    }
}

impl Subpalette {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let palette = DataId::read(reader)?;
        let offset = read_u8(reader)?;
        let num_colors = read_u8(reader)?;

        Ok(Self {
            palette,
            offset,
            num_colors,
        })
    }
}

impl crate::readers::ACDataType for Subpalette {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Subpalette::read(reader)
    }
}

impl TextureMapChange {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let old_tex_id = DataId::read(reader)?;
        let new_tex_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            old_tex_id,
            new_tex_id,
        })
    }
}

impl crate::readers::ACDataType for TextureMapChange {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        TextureMapChange::read(reader)
    }
}

impl AnimPartChange {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let part_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

impl crate::readers::ACDataType for AnimPartChange {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AnimPartChange::read(reader)
    }
}

