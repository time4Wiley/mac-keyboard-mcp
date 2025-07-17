//! Mac Keyboard MCP - AppleScript key code lookup server
//! 
//! This crate provides an MCP server for looking up macOS AppleScript key codes.

pub mod keycode;
pub mod mcp;
pub mod search;
pub mod utils;

pub use keycode::{KeyCode, KeyCategory, KEY_DATABASE};
pub use mcp::MacKeyboardServer;
pub use search::FuzzySearcher;

/// Current version of the MCP server
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod lib_tests;