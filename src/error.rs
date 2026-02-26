//! Error types for Neutral IPC client operations.
//!
//! This module defines the error types used throughout the Neutral IPC client
//! library, providing detailed error information for different failure scenarios.

use std::fmt;
use std::io;

/// Result type alias for Neutral IPC operations.
///
/// This type is used throughout the library to indicate operations that may fail
/// with a `NeutralIpcError`.
pub(crate) type Result<T> = std::result::Result<T, NeutralIpcError>;

/// Error types for Neutral IPC client operations.
///
/// This enum represents all possible error conditions that can occur when
/// communicating with the Neutral template server via IPC.
#[derive(Debug)]
pub enum NeutralIpcError {
    /// IO error from network operations, such as connection failures or read/write errors.
    Io(io::Error),
    /// Invalid header length received from the server.
    /// The expected header length is defined by `HEADER_LEN`.
    InvalidHeaderLength,
    /// Invalid or malformed response received from the server.
    InvalidResponse,
    /// Connection was closed unexpectedly during communication.
    ConnectionClosed,
    /// Invalid UTF-8 encoding in response content.
    InvalidUtf8,
    /// JSON parsing or serialization error.
    Json(serde_json::Error),
    /// MessagePack serialization error.
    MsgPackEncode(rmp_serde::encode::Error),
    /// MessagePack deserialization error.
    MsgPackDecode(rmp_serde::decode::Error),
}

impl fmt::Display for NeutralIpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeutralIpcError::Io(err) => write!(f, "IO error: {}", err),
            NeutralIpcError::InvalidHeaderLength => write!(f, "Invalid header length received"),
            NeutralIpcError::InvalidResponse => write!(f, "Invalid response from server"),
            NeutralIpcError::ConnectionClosed => write!(f, "Connection closed unexpectedly"),
            NeutralIpcError::InvalidUtf8 => write!(f, "Invalid UTF-8 encoding in response"),
            NeutralIpcError::Json(err) => write!(f, "JSON error: {}", err),
            NeutralIpcError::MsgPackEncode(err) => write!(f, "MsgPack encode error: {}", err),
            NeutralIpcError::MsgPackDecode(err) => write!(f, "MsgPack decode error: {}", err),
        }
    }
}

impl std::error::Error for NeutralIpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NeutralIpcError::Io(err) => Some(err),
            NeutralIpcError::Json(err) => Some(err),
            NeutralIpcError::MsgPackEncode(err) => Some(err),
            NeutralIpcError::MsgPackDecode(err) => Some(err),
            _ => None,
        }
    }
}

/// Convert from `io::Error` to `NeutralIpcError`.
///
/// This implementation allows IO errors to be automatically converted
/// to the appropriate `NeutralIpcError` variant.
impl From<io::Error> for NeutralIpcError {
    fn from(err: io::Error) -> Self {
        NeutralIpcError::Io(err)
    }
}

/// Convert from `serde_json::Error` to `NeutralIpcError`.
///
/// This implementation allows JSON errors to be automatically converted
/// to the appropriate `NeutralIpcError` variant.
impl From<serde_json::Error> for NeutralIpcError {
    fn from(err: serde_json::Error) -> Self {
        NeutralIpcError::Json(err)
    }
}

impl From<rmp_serde::encode::Error> for NeutralIpcError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        NeutralIpcError::MsgPackEncode(err)
    }
}

impl From<rmp_serde::decode::Error> for NeutralIpcError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        NeutralIpcError::MsgPackDecode(err)
    }
}
