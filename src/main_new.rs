use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

mod keycode;
mod search;
mod utils;
mod mcp_server;

use crate::mcp_server::MacKeyboardServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with stderr output
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Mac Keyboard MCP server v{}", env!("CARGO_PKG_VERSION"));

    // Create and serve the Mac Keyboard server
    let service = MacKeyboardServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("Serving error: {:?}", e);
        })?;

    // Wait for the service to complete
    service.waiting().await?;
    
    tracing::info!("Mac Keyboard MCP server stopped");
    Ok(())
}