mod generated {
    include!("generated/mod.rs");
}

pub use generated::enums;
pub use generated::gameactions;
pub use generated::gameevents;
pub use generated::messages;
pub use generated::types;
pub use generated::unified;

pub mod constants;
pub mod network;
pub mod parsing_context;
pub mod parsing_stats;
pub mod readers;

pub use parsing_context::{ParsingContext, ParseError, ErrorSeverity};
pub use parsing_stats::{ParsingStats, CategoryStats};
