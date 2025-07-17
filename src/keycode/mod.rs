pub mod database;
pub mod parser;
pub mod types;

pub use database::{KeyDatabase, KEY_DATABASE};
pub use parser::{parse_shortcut, KeyCombination, ParseError};
pub use types::{KeyCode, KeyCategory};