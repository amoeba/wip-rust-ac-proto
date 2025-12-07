// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl BlobFragments {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let id = read_u32(reader)?;
        let count = read_u16(reader)?;
        let size = read_u16(reader)?;
        let body_size = (size - 16) as u16;
        let index = read_u16(reader)?;
        let group = FragmentGroup::try_from(read_u16(reader)?)?;
        let data = read_vec::<u8>(reader, (body_size as usize))?;

        Ok(Self {
            sequence,
            id,
            count,
            size,
            index,
            group,
            data,
        })
    }
}

impl crate::readers::ACDataType for BlobFragments {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        BlobFragments::read(reader)
    }
}

impl ItemProfile {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let packed_amount = read_u32(reader)?;
        let amount = (packed_amount & 0x_ffffff) as i32;
        let pwd_type = (packed_amount >> 24) as i32;
        let object_id = ObjectId::read(reader)?;

        match pwd_type {
            -1 => {
                let weenie_description = PublicWeenieDesc::read(reader)?;

                Ok(Self::TypeNeg1 {
                    packed_amount,
                    object_id,
                    weenie_description,
                })
            }
            0x01 => {
                let old_weenie_description = OldPublicWeenieDesc::read(reader)?;

                Ok(Self::Type1 {
                    packed_amount,
                    object_id,
                    old_weenie_description,
                })
            }
            _ => Err(format!("Unknown PwdType value: {:#x}", pwd_type).into())
        }
    }
}

impl crate::readers::ACDataType for ItemProfile {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ItemProfile::read(reader)
    }
}

impl PackedMotionCommand {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let command_id = Command::try_from(read_u16(reader)?)?;
        let packed_sequence = read_u16(reader)?;
        let server_action_sequence = (packed_sequence & 0x7fff) as u16;
        let autonomous = ((packed_sequence >> 15) & 0x1) as u16;
        let speed = read_f32(reader)?;

        Ok(Self {
            command_id,
            packed_sequence,
            speed,
        })
    }
}

impl crate::readers::ACDataType for PackedMotionCommand {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        PackedMotionCommand::read(reader)
    }
}

impl RawMotionState {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 11) & 0x_f8) as u16;
        let mut current_holdkey = None;
        if (flags & 0x00000001) != 0 {
            current_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut current_style = None;
        if (flags & 0x00000002) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000004) != 0 {
            forward_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut forward_holdkey = None;
        if (flags & 0x0000008) != 0 {
            forward_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000010) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000020) != 0 {
            sidestep_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_holdkey = None;
        if (flags & 0x00000040) != 0 {
            sidestep_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000080) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000100) != 0 {
            turn_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut turn_holdkey = None;
        if (flags & 0x00000200) != 0 {
            turn_holdkey = Some(read_u32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000400) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, (command_list_length as usize))?;

        Ok(Self {
            flags,
            current_holdkey,
            current_style,
            forward_command,
            forward_holdkey,
            forward_speed,
            sidestep_command,
            sidestep_holdkey,
            sidestep_speed,
            turn_command,
            turn_holdkey,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for RawMotionState {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        RawMotionState::read(reader)
    }
}

impl InterpertedMotionState {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 7) & 0x7f) as u32;
        let mut current_style = None;
        if (flags & 0x00000001) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000002) != 0 {
            forward_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000008) != 0 {
            sidestep_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000020) != 0 {
            turn_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000004) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000010) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000040) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, (command_list_length as usize))?;

        Ok(Self {
            flags,
            current_style,
            forward_command,
            sidestep_command,
            turn_command,
            forward_speed,
            sidestep_speed,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for InterpertedMotionState {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        InterpertedMotionState::read(reader)
    }
}

impl DDDRevision {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let id_dat_file = read_u64(reader)?;
        let dat_file_type = (id_dat_file >> 32) as u32;
        let iteration = read_u32(reader)?;
        let ids_to_download = read_packable_list::<DataId>(reader)?;
        let ids_to_purge = read_packable_list::<DataId>(reader)?;

        Ok(Self {
            id_dat_file,
            iteration,
            ids_to_download,
            ids_to_purge,
        })
    }
}

impl crate::readers::ACDataType for DDDRevision {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        DDDRevision::read(reader)
    }
}

