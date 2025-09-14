//! Constants for Neutral IPC record protocol.
//!
//! This module defines the constants used in the Neutral IPC record protocol,
//! which is used for communication between the client and the Neutral template server.

/// Reserved field in the protocol header.
///
/// This field is reserved for future use and should always be set to 0.
pub const RESERVED: u8 = 0;

/// Length of the protocol header in bytes.
///
/// The header contains control information and metadata about the payload.
/// This constant defines the total size of the header structure.
pub const HEADER_LEN: usize = 12;

/// Control code for template parsing operations.
///
/// This control code is used when the client wants to parse and process
/// a template with the provided data.
pub const CTRL_PARSE_TEMPLATE: u8 = 10;

/// Status code indicating successful operation.
///
/// This status code is returned by the server when the requested operation
/// completed successfully without any errors.
pub const CTRL_STATUS_OK: u8 = 0;

/// Status code indicating operation failure.
///
/// This status code is returned by the server when the requested operation
/// failed due to an error condition.
pub const CTRL_STATUS_KO: u8 = 1;

/// Content type identifier for JSON data.
///
/// This constant indicates that the payload contains JSON-formatted data.
pub const CONTENT_JSON: u8 = 10;

/// Content type identifier for file path data.
///
/// This constant indicates that the payload contains file path information.
pub const CONTENT_PATH: u8 = 20;

/// Content type identifier for text data.
///
/// This constant indicates that the payload contains plain text data.
pub const CONTENT_TEXT: u8 = 30;

/// Content type identifier for binary data.
///
/// This constant indicates that the payload contains binary data.
pub const CONTENT_BIN: u8 = 40;
