//! IPC client implementation for communicating with the Neutral template server.
//!
//! This module provides the core client functionality that handles TCP connections,
//! protocol encoding/decoding, and communication with the Neutral server.

use serde_json::Value;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::config::NeutralIpcConfig;
use crate::constants::*;
use crate::error::{NeutralIpcError, Result};
use crate::record::NeutralIpcRecord;

/// IPC client for communicating with the Neutral template server.
///
/// This client handles the low-level protocol communication, including:
/// - TCP connection establishment
/// - Protocol record encoding/decoding
/// - Request/response handling
/// - Error handling for network operations
pub(crate) struct NeutralIpcClient {
    /// Control byte indicating the operation type
    control: u8,
    /// Format identifier for the first content field
    format1: u8,
    /// First content field (JSON or MsgPack schema)
    content1: Vec<u8>,
    /// Format identifier for the second content field
    format2: u8,
    /// Second content field (typically template content)
    content2: String,
    /// Parsed result from the server response
    pub(crate) result: HashMap<String, Value>,
}

impl NeutralIpcClient {
    /// Create a new IPC client with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `control` - Control byte indicating the operation type (e.g., `CTRL_PARSE_TEMPLATE`)
    /// * `format1` - Format identifier for the first content field (e.g., `CONTENT_JSON`)
    /// * `content1` - First content field, typically a JSON schema
    /// * `format2` - Format identifier for the second content field (e.g., `CONTENT_TEXT`)
    /// * `content2` - Second content field, typically template content
    pub(crate) fn new(control: u8, format1: u8, content1: &[u8], format2: u8, content2: &str) -> Self {
        Self {
            control,
            format1,
            content1: content1.to_vec(),
            format2,
            content2: content2.to_string(),
            result: HashMap::new(),
        }
    }

    /// Start the IPC communication with the Neutral server.
    ///
    /// This method:
    /// 1. Loads configuration for host, port, timeout, and buffer size
    /// 2. Establishes a TCP connection to the configured server
    /// 3. Sets read/write timeouts based on configuration
    /// 4. Encodes and sends the request record
    /// 5. Reads and decodes the response
    /// 6. Stores the parsed result
    ///
    /// # Returns
    ///
    /// A reference to the parsed result map containing the server's response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Connection to the server fails
    /// - Network I/O operations fail
    /// - The server response is invalid or malformed
    /// - UTF-8 decoding of response content fails
    pub(crate) fn start(&mut self) -> Result<&HashMap<String, Value>> {
        let config = NeutralIpcConfig::new();
        let host = config.get_host();
        let port = config.get_port();
        let timeout = config.get_timeout();
        let buffer_size = config.get_buffer_size();

        let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
        stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;
        stream.set_write_timeout(Some(Duration::from_secs(timeout as u64)))?;

        let request = NeutralIpcRecord::encode_record(
            self.control,
            self.format1,
            &self.content1,
            self.format2,
            self.content2.as_bytes(),
        );
        stream.write_all(&request)?;

        let mut response_header = vec![0u8; HEADER_LEN];
        stream.read_exact(&mut response_header)?;

        let response = NeutralIpcRecord::decode_header(&response_header)?;
        let length1 = response.get("length-1")
            .and_then(|v| v.as_u64())
            .ok_or(NeutralIpcError::InvalidResponse)? as usize;
        let length2 = response.get("length-2")
            .and_then(|v| v.as_u64())
            .ok_or(NeutralIpcError::InvalidResponse)? as usize;

        let content1 = self.read_content(&mut stream, length1, buffer_size)?;
        let content2 = self.read_content(&mut stream, length2, buffer_size)?;

        self.result = NeutralIpcRecord::decode_record(&response_header, &content1, &content2)?;

        Ok(&self.result)
    }

    /// Read content from the TCP stream in chunks.
    ///
    /// This method reads exactly `length` bytes from the stream, handling
    /// partial reads and buffering. It ensures that the entire content is
    /// read even if the data arrives in multiple chunks.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to read from
    /// * `length` - The exact number of bytes to read
    /// * `buffer_size` - The maximum size of each read chunk
    ///
    /// # Returns
    ///
    /// The read content as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The connection is closed before all data is read
    /// - The content cannot be decoded as valid UTF-8
    fn read_content(&self, stream: &mut TcpStream, length: usize, buffer_size: usize) -> Result<String> {
        if length == 0 {
            return Ok(String::new());
        }

        let mut chunks = Vec::new();
        let mut remaining = length;

        while remaining > 0 {
            let chunk_size = std::cmp::min(buffer_size, remaining);
            let mut chunk = vec![0u8; chunk_size];
            let bytes_read = stream.read(&mut chunk)?;

            if bytes_read == 0 {
                return Err(NeutralIpcError::ConnectionClosed);
            }

            chunks.extend_from_slice(&chunk[..bytes_read]);
            remaining -= bytes_read;
        }

        String::from_utf8(chunks).map_err(|_| NeutralIpcError::InvalidUtf8)
    }
}

/// Check if the Neutral server is available and responding.
///
/// This function performs a lightweight availability check by:
/// 1. Attempting to connect to the server with a 1-second timeout
/// 2. Sending a minimal valid request
/// 3. Reading the response header to verify the server is responsive
///
/// # Returns
///
/// `true` if the server is available and responding correctly, `false` otherwise.
///
/// # Note
///
/// This function is primarily used in tests, but may be useful for runtime server availability checks.
pub fn is_server_available() -> bool {
    let config = NeutralIpcConfig::new();
    let host = config.get_host();
    let port = config.get_port();

    match TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        std::time::Duration::from_secs(1)
    ) {
        Ok(mut stream) => {

            let minimal_request = NeutralIpcRecord::encode_record(
                CTRL_PARSE_TEMPLATE,
                CONTENT_JSON,
                b"{}",
                CONTENT_TEXT,
                b""
            );

            stream.set_read_timeout(Some(std::time::Duration::from_secs(1))).ok();
            stream.set_write_timeout(Some(std::time::Duration::from_secs(1))).ok();

            match stream.write_all(&minimal_request) {
                Ok(_) => {
                    let mut header_buffer = [0u8; HEADER_LEN];
                    stream.read_exact(&mut header_buffer).is_ok()
                },
                Err(_) => false
            }
        },
        Err(_) => false
    }
}
