use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, BufReader, Write};
use tracing::{debug, error, info};
use tracing_subscriber;

mod keycode;
mod mcp;
mod search;
mod utils;

use crate::mcp::MacKeyboardServer;

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[serde(default = "default_jsonrpc")]
    jsonrpc: String,
    #[serde(default)]
    id: Option<Value>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    params: Option<Value>,
}

fn default_jsonrpc() -> String {
    "2.0".to_string()
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr so it doesn't interfere with stdio
    tracing_subscriber::fmt()
        .with_env_filter("mac_keyboard_mcp=debug")
        .with_writer(io::stderr)
        .init();
    
    info!("Starting Mac Keyboard MCP server v{}", env!("CARGO_PKG_VERSION"));
    
    // Create the MCP server
    let server = MacKeyboardServer::new()?;
    
    // Read from stdin and write to stdout
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut stdout = io::stdout();
    
    info!("MCP server ready, listening on stdio...");
    
    let mut initialized = false;
    
    // Process JSON-RPC messages
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("Error reading from stdin: {}", e);
                break;
            }
        };
        
        if line.trim().is_empty() {
            continue;
        }
        
        debug!("Received: {}", line);
        
        // Parse the JSON-RPC request
        match serde_json::from_str::<JsonRpcRequest>(&line) {
            Ok(request) => {
                // Track initialization state
                if let Some(ref method) = request.method {
                    if method == "initialized" {
                        initialized = true;
                        debug!("Client initialization complete");
                    }
                }
                
                let response = handle_request(&server, request).await;
                
                // Only send response if it's not a notification (has an ID)
                if let Some(response) = response {
                    let response_str = serde_json::to_string(&response)?;
                    
                    debug!("Sending: {}", response_str);
                    writeln!(stdout, "{}", response_str)?;
                    stdout.flush()?;
                } else {
                    debug!("No response for notification");
                }
                
                // Continue running after initialization
                if initialized {
                    debug!("Server is initialized and ready for requests");
                }
            }
            Err(e) => {
                error!("Failed to parse request: {}", e);
                error!("Raw request was: {}", line);
                
                // Try to extract ID from raw JSON for error response
                let id = if let Ok(raw_json) = serde_json::from_str::<Value>(&line) {
                    raw_json.get("id").cloned()
                } else {
                    None
                };
                
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "Parse error".to_string(),
                        data: Some(json!({
                            "details": e.to_string(),
                            "raw_request": line
                        })),
                    }),
                };
                
                let response_str = serde_json::to_string(&error_response)?;
                writeln!(stdout, "{}", response_str)?;
                stdout.flush()?;
            }
        }
    }
    
    info!("Client disconnected, server shutting down");
    Ok(())
}

async fn handle_request(server: &MacKeyboardServer, request: JsonRpcRequest) -> Option<JsonRpcResponse> {
    // Handle case where method might be None
    let method = match request.method {
        Some(m) => m,
        None => {
            return Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid Request: missing method".to_string(),
                    data: None,
                }),
            });
        }
    };
    
    match method.as_str() {
        "initialize" => {
            // MCP initialization
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {},
                        "resources": {},
                        "prompts": {},
                        "logging": {}
                    },
                    "serverInfo": {
                        "name": "mac-keyboard-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
                error: None,
            }
        }
        "initialized" => {
            // Client confirms initialization
            // Even though this is typically a notification, Claude expects a response
            debug!("Client initialized");
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,  // Keep the ID from request
                result: Some(json!({})),  // Empty result
                error: None,
            }
        }
        "tools/list" => {
            // List available tools
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "tools": server.list_tools()
                })),
                error: None,
            }
        }
        "tools/call" => {
            // Call a tool
            if let Some(params) = request.params {
                if let Ok(tool_call) = serde_json::from_value::<ToolCall>(params) {
                    match server.call_tool(&tool_call.name, tool_call.arguments).await {
                        Ok(result) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(json!({ 
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result).unwrap()
                                }]
                            })),
                            error: None,
                        },
                        Err(e) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Tool execution error".to_string(),
                                data: Some(json!({"details": e.to_string()})),
                            }),
                        },
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: None,
                        }),
                    }
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing params".to_string(),
                        data: None,
                    }),
                }
            }
        }
        "resources/list" => {
            // List available resources
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "resources": server.list_resources()
                })),
                error: None,
            }
        }
        "resources/read" => {
            // Read a resource
            if let Some(params) = request.params {
                if let Ok(resource_read) = serde_json::from_value::<ResourceRead>(params) {
                    match server.read_resource(&resource_read.uri).await {
                        Ok(result) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(json!({ 
                                "contents": [{
                                    "uri": resource_read.uri,
                                    "mimeType": "application/json",
                                    "text": serde_json::to_string_pretty(&result).unwrap()
                                }]
                            })),
                            error: None,
                        },
                        Err(e) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Resource read error".to_string(),
                                data: Some(json!({"details": e.to_string()})),
                            }),
                        },
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: None,
                        }),
                    }
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing params".to_string(),
                        data: None,
                    }),
                }
            }
        }
        _ => {
            // Unknown method
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", method),
                    data: None,
                }),
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    name: String,
    arguments: Value,
}

#[derive(Debug, Deserialize)]
struct ResourceRead {
    uri: String,
}