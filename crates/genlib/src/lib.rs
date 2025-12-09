pub mod codegen;
pub mod identifiers;
pub mod types;
pub mod util;

mod field_gen;
mod generation;
mod type_utils;
mod xml;
mod xml_parser;

pub use generation::{
    GenerateSource, GeneratedCode, GeneratedFile, GenerationContext, ReaderContext, generate,
    generate_and_merge, generate_with_source,
};
pub use type_utils::get_rust_type;
pub use types::ProtocolCategory;
