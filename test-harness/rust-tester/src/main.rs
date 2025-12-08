use anyhow::Result;
use serde_json;
use std::env;
use std::fs::{self, File};
use std::io::{Cursor, Read};
use std::path::Path;

// Import specific message types we want to support
use acprotocol::messages::c2s::inventory_get_and_wield_item::InventoryGetAndWieldItem;
use acprotocol::messages::c2s::inventory_put_item_in_container::InventoryPutItemInContainer;
use acprotocol::messages::c2s::item_appraise::ItemAppraise;
use acprotocol::messages::s2c::communication_textbox_string::CommunicationTextboxString;
use acprotocol::messages::s2c::effects_play_script_type::EffectsPlayScriptType;
use acprotocol::messages::s2c::effects_sound_event::EffectsSoundEvent;
use acprotocol::messages::s2c::inventory_pickup_event::InventoryPickupEvent;
use acprotocol::messages::s2c::item_obj_desc_event::ItemObjDescEvent;
use acprotocol::messages::s2c::item_wear_item::ItemWearItem;
use acprotocol::messages::s2c::magic_dispel_enchantment::MagicDispelEnchantment;
use acprotocol::messages::s2c::magic_update_enchantment::MagicUpdateEnchantment;
use acprotocol::messages::s2c::qualities_private_update_attribute2nd_level::QualitiesPrivateUpdateAttribute2ndLevel;

#[derive(Debug, Clone)]
struct PacketInfo {
    direction: Direction,
    opcode: u32,
    name: String,
}

#[derive(Debug, Clone)]
enum Direction {
    C2S,
    S2C,
}

impl PacketInfo {
    fn from_filename(filename: &str) -> Result<Self> {
        // Parse filename like: "0681_ClientToServer_63409_Item_Appraise.bin"
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() < 4 {
            return Err(anyhow::anyhow!("Invalid filename format: {}", filename));
        }

        let direction = match parts[1] {
            "ClientToServer" => Direction::C2S,
            "ServerToClient" => Direction::S2C,
            _ => return Err(anyhow::anyhow!("Unknown direction: {}", parts[1])),
        };

        let opcode = parts[2].parse::<u32>()?;
        let name = parts[3..].join("_").replace(".bin", "");

        Ok(PacketInfo {
            direction,
            opcode,
            name,
        })
    }
}

fn process_file(filename: &str) -> Result<()> {
    println!("Processing file: {}", filename);

    // Parse packet info from filename
    let packet_info = PacketInfo::from_filename(filename)?;
    println!("Packet info: {:?}", packet_info);

    // Read binary data
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("Read {} bytes from {}", buffer.len(), filename);

    // Print hex dump for reference
    println!("Hex dump:");
    for (i, byte) in buffer.iter().enumerate() {
        if i % 16 == 0 {
            print!("{:04x}: ", i);
        }
        print!("{:02x} ", byte);
        if i % 16 == 15 {
            println!();
        }
    }
    if buffer.len() % 16 != 0 {
        println!();
    }

    // WIP: I think we want to skip the 8 byte preamble
    let mut cursor = Cursor::new(&buffer);
    cursor.set_position(8);

    // These files contain just the message payload, no packet headers
    // Try to parse message based on direction and name
    let result = match (&packet_info.direction, packet_info.name.as_str()) {
        (Direction::C2S, "Item_Appraise") => match ItemAppraise::read(&mut cursor) {
            Ok(msg) => Some(serde_json::to_value(msg)?),
            Err(e) => {
                eprintln!("Failed to parse ItemAppraise: {}", e);
                None
            }
        },
        (Direction::C2S, "Inventory_PutItemInContainer") => {
            match InventoryPutItemInContainer::read(&mut cursor) {
                Ok(msg) => Some(serde_json::to_value(msg)?),
                Err(e) => {
                    eprintln!("Failed to parse InventoryPutItemInContainer: {}", e);
                    None
                }
            }
        }
        (Direction::C2S, "Inventory_GetAndWieldItem") => {
            match InventoryGetAndWieldItem::read(&mut cursor) {
                Ok(msg) => Some(serde_json::to_value(msg)?),
                Err(e) => {
                    eprintln!("Failed to parse InventoryGetAndWieldItem: {}", e);
                    None
                }
            }
        }
        (Direction::S2C, "Qualities_PrivateUpdateAttribute2ndLevel") => {
            match QualitiesPrivateUpdateAttribute2ndLevel::read(&mut cursor) {
                Ok(msg) => Some(serde_json::to_value(msg)?),
                Err(e) => {
                    eprintln!(
                        "Failed to parse QualitiesPrivateUpdateAttribute2ndLevel: {}",
                        e
                    );
                    None
                }
            }
        }
        (Direction::S2C, "Item_ObjDescEvent") => match ItemObjDescEvent::read(&mut cursor) {
            Ok(msg) => Some(serde_json::to_value(msg)?),
            Err(e) => {
                eprintln!("Failed to parse ItemObjDescEvent: {}", e);
                None
            }
        },
        (Direction::S2C, "Communication_TextboxString") => {
            match CommunicationTextboxString::read(&mut cursor) {
                Ok(msg) => Some(serde_json::to_value(msg)?),
                Err(e) => {
                    eprintln!("Failed to parse CommunicationTextboxString: {}", e);
                    None
                }
            }
        }
        (Direction::S2C, "Magic_DispelEnchantment") => {
            match MagicDispelEnchantment::read(&mut cursor) {
                Ok(msg) => Some(serde_json::to_value(msg)?),
                Err(e) => {
                    eprintln!("Failed to parse MagicDispelEnchantment: {}", e);
                    None
                }
            }
        }
        (Direction::S2C, "Magic_UpdateEnchantment") => {
            match MagicUpdateEnchantment::read(&mut cursor) {
                Ok(msg) => {
                    println!("Successfully parsed MagicUpdateEnchantment:");
                    Some(serde_json::to_value(msg)?)
                }
                Err(e) => {
                    eprintln!("Failed to parse MagicUpdateEnchantment: {}", e);
                    None
                }
            }
        }
        (Direction::S2C, "Item_WearItem") => match ItemWearItem::read(&mut cursor) {
            Ok(msg) => {
                println!("Successfully parsed ItemWearItem:");
                Some(serde_json::to_value(msg)?)
            }
            Err(e) => {
                eprintln!("Failed to parse ItemWearItem: {}", e);
                None
            }
        },
        (Direction::S2C, "Effects_PlayScriptType") => {
            match EffectsPlayScriptType::read(&mut cursor) {
                Ok(msg) => {
                    println!("Successfully parsed EffectsPlayScriptType:");
                    Some(serde_json::to_value(msg)?)
                }
                Err(e) => {
                    eprintln!("Failed to parse EffectsPlayScriptType: {}", e);
                    None
                }
            }
        }
        (Direction::S2C, "Effects_SoundEvent") => match EffectsSoundEvent::read(&mut cursor) {
            Ok(msg) => {
                println!("Successfully parsed EffectsSoundEvent:");
                Some(serde_json::to_value(msg)?)
            }
            Err(e) => {
                eprintln!("Failed to parse EffectsSoundEvent: {}", e);
                None
            }
        },
        (Direction::S2C, "Inventory_PickupEvent") => {
            match InventoryPickupEvent::read(&mut cursor) {
                Ok(msg) => {
                    println!("Successfully parsed InventoryPickupEvent:");
                    Some(serde_json::to_value(msg)?)
                }
                Err(e) => {
                    eprintln!("Failed to parse InventoryPickupEvent: {}", e);
                    None
                }
            }
        }
        _ => {
            println!(
                "Unsupported message type: {} ({:?})",
                packet_info.name, packet_info.direction
            );
            None
        }
    };

    // Print JSON if we successfully parsed
    if let Some(value) = result {
        match serde_json::to_string_pretty(&value) {
            Ok(json) => println!("JSON: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
    }

    println!("Successfully processed file: {}", filename);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let path = Path::new(dir_path);

    if !path.exists() || !path.is_dir() {
        eprintln!("Error: {} is not a valid directory", dir_path);
        std::process::exit(1);
    }

    println!("Processing directory: {}", dir_path);

    // Get all .bin files in the directory
    let entries = fs::read_dir(path)?;
    let mut bin_files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "bin") {
            if let Some(path_str) = path.to_str() {
                bin_files.push(path_str.to_string());
            }
        }
    }

    if bin_files.is_empty() {
        println!("No .bin files found in directory: {}", dir_path);
        return Ok(());
    }

    println!("Found {} .bin files", bin_files.len());

    // Process each file
    for filename in &bin_files {
        println!("\n{}", "=".repeat(80));
        if let Err(e) = process_file(filename) {
            eprintln!("Error processing {}: {}", filename, e);
        }
    }

    println!("\nProcessed {} files", bin_files.len());
    Ok(())
}
