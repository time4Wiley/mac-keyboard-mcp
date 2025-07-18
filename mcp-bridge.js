#!/usr/bin/env node
/**
 * Bridge between Claude Desktop and the Rust MCP server
 * This handles any protocol mismatches and validation issues
 */

const { spawn } = require('child_process');
const readline = require('readline');

// Path to the Rust server
const RUST_SERVER = '/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp';

// Start the Rust server
const rustServer = spawn(RUST_SERVER, [], {
  stdio: ['pipe', 'pipe', 'inherit']
});

// Create readline interfaces
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

const rustRl = readline.createInterface({
  input: rustServer.stdout,
  terminal: false
});

// Forward requests from Claude to Rust server
rl.on('line', (line) => {
  try {
    const request = JSON.parse(line);
    
    // Log for debugging
    process.stderr.write(`Bridge: Claude -> Rust: ${line}\n`);
    
    // Forward to Rust server
    rustServer.stdin.write(line + '\n');
  } catch (e) {
    process.stderr.write(`Bridge: Error parsing request: ${e}\n`);
    // Send error response
    const errorResponse = {
      jsonrpc: "2.0",
      id: null,
      error: {
        code: -32700,
        message: "Parse error",
        data: e.toString()
      }
    };
    console.log(JSON.stringify(errorResponse));
  }
});

// Forward responses from Rust server to Claude
rustRl.on('line', (line) => {
  try {
    const response = JSON.parse(line);
    
    // Log for debugging
    process.stderr.write(`Bridge: Rust -> Claude: ${line}\n`);
    
    // Forward to Claude
    console.log(JSON.stringify(response));
  } catch (e) {
    process.stderr.write(`Bridge: Error parsing response: ${e}\n`);
  }
});

// Handle server exit
rustServer.on('close', (code) => {
  process.stderr.write(`Bridge: Rust server exited with code ${code}\n`);
  process.exit(code);
});

// Handle errors
rustServer.on('error', (err) => {
  process.stderr.write(`Bridge: Failed to start Rust server: ${err}\n`);
  process.exit(1);
});

process.on('SIGINT', () => {
  rustServer.kill();
  process.exit(0);
});

process.on('SIGTERM', () => {
  rustServer.kill();
  process.exit(0);
});