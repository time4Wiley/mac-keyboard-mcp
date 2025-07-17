use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod keycode;
mod mcp;
mod search;
mod utils;

use crate::mcp::MacKeyboardServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("mac_keyboard_mcp=debug,mcp_sdk=info")
        .init();
    
    info!("Starting Mac Keyboard MCP server v{}", env!("CARGO_PKG_VERSION"));
    
    // Create and run the MCP server
    let server = MacKeyboardServer::new()?;
    
    // TODO: Replace with actual MCP server run when rmcp is available
    info!("Server initialized successfully");
    info!("Waiting for MCP connections on stdio...");
    
    // For now, just keep the process running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    
    Ok(())
}