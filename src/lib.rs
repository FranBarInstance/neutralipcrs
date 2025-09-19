//! Neutral IPC Template package.
//!
//! This crate provides a Rust client for the Neutral template engine,
//! which processes templates via Inter-Process Communication (IPC).
//! The client communicates with a Neutral server to render templates
//! with JSON data schemas.
//!
//! # Examples
//!
//! ```no_run
//! use neutralipcrs::NeutralIpcTemplate;
//! use serde_json::json;
//!
//! let schema = json!({
//!     "data": {
//!         "text": "World"
//!     }
//! });
//!
//! let mut template = NeutralIpcTemplate::from_src_value("Hello {:;text:}!", schema).unwrap();
//! let result = template.render().unwrap();
//!
//! println!("{}", result); // Output: "Hello World!"
//! ```
//!
//! # Configuration
//!
//! The client reads the server configuration from `/etc/neutral-ipc-cfg.json` to
//! determine connection settings (host and port).


pub mod config;
pub mod constants;
pub mod template;
pub mod client;
pub(crate) mod error;
pub(crate) mod record;

pub use config::NeutralIpcConfig;
pub use constants::*;
pub use error::NeutralIpcError;
pub use template::NeutralIpcTemplate;
