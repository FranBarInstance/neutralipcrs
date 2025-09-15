//! Neutral IPC record handling for protocol messages.
//!
//! This module provides functionality to encode and decode Neutral IPC protocol records,
//! which are the fundamental unit of communication between the client and server.


// ============================================
// Neutral IPC record version 0 (draft version)
// ============================================
//
// HEADER:
//
// \x00              // reserved
// \x00              // control (action/status) (10 = parse template)
// \x00              // content-format 1 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
// \x00\x00\x00\x00  // content-length 1 big endian byte order
// \x00              // content-format 2 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
// \x00\x00\x00\x00  // content-length 2 big endian byte order (can be zero)
//
// All text utf8

use serde_json::Value;
use std::collections::HashMap;

use crate::constants::*;
use crate::error::{NeutralIpcError, Result};

/// Neutral IPC record for encoding/decoding protocol messages.
///
/// This struct provides static methods for working with Neutral IPC protocol records.
/// Records consist of a fixed-length header followed by optional content payloads.
#[derive(Debug, Clone)]
pub(crate) struct NeutralIpcRecord;

impl NeutralIpcRecord {
    /// Decode an IPC record header into a structured format.
    ///
    /// # Arguments
    ///
    /// * `record_header` - A byte slice containing exactly `HEADER_LEN` bytes
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the decoded header fields:
    /// - `reserved`: Reserved field read from the header
    /// - `control`: Control code for the operation
    /// - `format-1`: Format identifier for the first content block
    /// - `length-1`: Length of the first content block in bytes
    /// - `format-2`: Format identifier for the second content block
    /// - `length-2`: Length of the second content block in bytes
    ///
    /// # Errors
    ///
    /// Returns `NeutralIpcError::InvalidHeaderLength` if the header length is incorrect.
    pub(crate) fn decode_header(record_header: &[u8]) -> Result<HashMap<String, Value>> {
        if record_header.len() != HEADER_LEN {
            return Err(NeutralIpcError::InvalidHeaderLength);
        }

        let reserved = record_header[0];
        let control = record_header[1];
        let format1 = record_header[2];

        let length1 = u32::from_be_bytes([
            record_header[3], record_header[4], record_header[5], record_header[6]
        ]);

        let format2 = record_header[7];

        let length2 = u32::from_be_bytes([
            record_header[8], record_header[9], record_header[10], record_header[11]
        ]);

        let mut header = HashMap::new();
        header.insert("reserved".to_string(), Value::Number(reserved.into()));
        header.insert("control".to_string(), Value::Number(control.into()));
        header.insert("format-1".to_string(), Value::Number(format1.into()));
        header.insert("length-1".to_string(), Value::Number(length1.into()));
        header.insert("format-2".to_string(), Value::Number(format2.into()));
        header.insert("length-2".to_string(), Value::Number(length2.into()));

        Ok(header)
    }

    /// Encode an IPC record header from individual components.
    ///
    /// # Arguments
    ///
    /// * `control` - Control code for the operation
    /// * `format1` - Format identifier for the first content block
    /// * `length1` - Length of the first content block in bytes
    /// * `format2` - Format identifier for the second content block
    /// * `length2` - Length of the second content block in bytes
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encoded header with exactly `HEADER_LEN` bytes.
    pub(crate) fn encode_header(control: u8, format1: u8, length1: u32, format2: u8, length2: u32) -> Vec<u8> {
        let mut header = Vec::with_capacity(HEADER_LEN);
        header.push(RESERVED);
        header.push(control);
        header.push(format1);
        header.extend_from_slice(&length1.to_be_bytes());
        header.push(format2);
        header.extend_from_slice(&length2.to_be_bytes());
        header
    }

    /// Encode a complete IPC record with header and content.
    ///
    /// # Arguments
    ///
    /// * `control` - Control code for the operation
    /// * `format1` - Format identifier for the first content block
    /// * `content1` - Content for the first block as a string
    /// * `format2` - Format identifier for the second content block
    /// * `content2` - Content for the second block as a string
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the complete record with header and both content blocks.
    pub(crate) fn encode_record(control: u8, format1: u8, content1: &str, format2: u8, content2: &str) -> Vec<u8> {
        let content1_bytes = content1.as_bytes();
        let content2_bytes = content2.as_bytes();
        let length1 = content1_bytes.len() as u32;
        let length2 = content2_bytes.len() as u32;

        let mut record = Self::encode_header(control, format1, length1, format2, length2);
        record.extend_from_slice(content1_bytes);
        record.extend_from_slice(content2_bytes);
        record
    }

    /// Decode a complete IPC record from header and content components.
    ///
    /// # Arguments
    ///
    /// * `header` - The record header bytes
    /// * `content1` - The first content block as a string
    /// * `content2` - The second content block as a string
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the complete decoded record including:
    /// - `reserved`: Reserved field (hardcoded to the RESERVED constant)
    /// - `control`: Control code read from the header
    /// - `format-1`: Format identifier read from the header
    /// - `content-1`: The first content block
    /// - `format-2`: Format identifier read from the header
    /// - `content-2`: The second content block
    ///
    /// # Errors
    ///
    /// Returns `NeutralIpcError::InvalidHeaderLength` if the header length is incorrect.
    pub(crate) fn decode_record(header: &[u8], content1: &str, content2: &str) -> Result<HashMap<String, Value>> {
        let _header_map = Self::decode_header(header)?;

        let mut record = HashMap::new();
        record.insert("reserved".to_string(), Value::Number(RESERVED.into()));
        record.insert("control".to_string(), Value::Number(header[1].into()));
        record.insert("format-1".to_string(), Value::Number(header[2].into()));
        record.insert("content-1".to_string(), Value::String(content1.to_string()));
        record.insert("format-2".to_string(), Value::Number(header[7].into()));
        record.insert("content-2".to_string(), Value::String(content2.to_string()));

        Ok(record)
    }
}
