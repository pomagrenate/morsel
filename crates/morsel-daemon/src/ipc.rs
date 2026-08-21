//! IPC (Inter-Process Communication) module for morsel daemon.
//!
//! This module provides the protocol and communication layer between
//! the daemon and CLI clients.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use thiserror::Error;

/// IPC error types.
#[derive(Error, Debug)]
pub enum IpcError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Invalid request: {0}")]
    #[allow(dead_code)]
    InvalidRequest(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// IPC request type.
#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    /// Get daemon status
    Status,
    /// Stop the daemon
    Stop,
    /// Get clipboard history
    GetHistory { limit: usize },
    /// Search clipboard history
    Search { query: String, limit: usize },
    /// Get a specific item
    GetItem { id: String },
    /// Add an item to history
    AddItem { content: String },
    /// Delete an item
    DeleteItem { id: String },
    /// Clear all history
    ClearHistory,
}

/// IPC response type.
#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse {
    /// Success response with optional data
    Success { data: Option<serde_json::Value> },
    /// Error response
    Error { message: String },
}

/// IPC server for handling client connections.
pub struct IpcServer {
    listener: TcpListener,
}

impl IpcServer {
    /// Create a new IPC server bound to the specified address.
    pub fn new(addr: &str) -> Result<Self, IpcError> {
        let listener = TcpListener::bind(addr)
            .map_err(|e| IpcError::ConnectionError(format!("Failed to bind to {}: {}", addr, e)))?;
        Ok(Self { listener })
    }

    /// Accept a new client connection.
    pub fn accept(&self) -> Result<ClientConnection, IpcError> {
        let (stream, addr) = self.listener.accept()
            .map_err(|e| IpcError::ConnectionError(format!("Failed to accept connection: {}", e)))?;
        Ok(ClientConnection::new(stream, addr.to_string()))
    }

    /// Get the local address the server is bound to.
    #[allow(dead_code)]
    pub fn local_addr(&self) -> Result<std::net::SocketAddr, IpcError> {
        self.listener.local_addr()
            .map_err(|e| IpcError::ConnectionError(format!("Failed to get local address: {}", e)))
    }
}

/// IPC client connection.
pub struct ClientConnection {
    stream: TcpStream,
    addr: String,
}

impl ClientConnection {
    /// Create a new client connection.
    fn new(stream: TcpStream, addr: String) -> Self {
        Self { stream, addr }
    }

    /// Get the remote address.
    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// Read a request from the client.
    pub fn read_request(&mut self) -> Result<IpcRequest, IpcError> {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        
        let request: IpcRequest = serde_json::from_str(line.trim())
            .map_err(|e| IpcError::SerializationError(format!("Failed to parse request: {}", e)))?;
        
        Ok(request)
    }

    /// Write a response to the client.
    pub fn write_response(&mut self, response: &IpcResponse) -> Result<(), IpcError> {
        let response_json = serde_json::to_string(response)
            .map_err(|e| IpcError::SerializationError(format!("Failed to serialize response: {}", e)))?;
        
        let mut writer = BufWriter::new(&self.stream);
        writeln!(writer, "{}", response_json)?;
        writer.flush()?;
        
        Ok(())
    }
}

/// IPC client for connecting to the daemon.
#[allow(dead_code)]
pub struct IpcClient {
    stream: TcpStream,
}

#[allow(dead_code)]
impl IpcClient {
    /// Connect to the IPC server.
    pub fn connect(addr: &str) -> Result<Self, IpcError> {
        let stream = TcpStream::connect(addr)
            .map_err(|e| IpcError::ConnectionError(format!("Failed to connect to {}: {}", addr, e)))?;
        Ok(Self { stream })
    }

    /// Send a request and wait for a response.
    pub fn send_request(&mut self, request: &IpcRequest) -> Result<IpcResponse, IpcError> {
        let request_json = serde_json::to_string(request)
            .map_err(|e| IpcError::SerializationError(format!("Failed to serialize request: {}", e)))?;
        
        let mut writer = BufWriter::new(&self.stream);
        writeln!(writer, "{}", request_json)?;
        writer.flush()?;
        
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        
        let response: IpcResponse = serde_json::from_str(line.trim())
            .map_err(|e| IpcError::SerializationError(format!("Failed to parse response: {}", e)))?;
        
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = IpcRequest::Status;
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: IpcRequest = serde_json::from_str(&json).unwrap();
        matches!(deserialized, IpcRequest::Status);
    }

    #[test]
    fn test_response_serialization() {
        let response = IpcResponse::Success { data: None };
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: IpcResponse = serde_json::from_str(&json).unwrap();
        matches!(deserialized, IpcResponse::Success { data: None });
    }
}
