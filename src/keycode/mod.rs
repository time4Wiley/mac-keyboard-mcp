pub mod database;
pub mod parser;
pub mod secondary_functions;
pub mod types;

pub use database::{KeyDatabase, KEY_DATABASE};
pub use parser::{parse_shortcut, KeyCombination, ParseError};
pub use secondary_functions::{get_secondary_function, is_secondary_function_query};
pub use types::{KeyCode, KeyCategory};