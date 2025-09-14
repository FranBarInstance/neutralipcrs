//! Configuration module for Neutral IPC client.
//! Reads configuration from /etc/neutral-ipc-cfg.json or uses default values.
//! neutral-ipc-cfg.json is the configuration file used by the IPC server.

use serde_json::Value;
use std::fs;
use std::path::Path;

/// Configuration class for Neutral IPC client.
///
/// This struct provides configuration values by reading from a JSON file
/// at /etc/neutral-ipc-cfg.json. If the file doesn't exist or values are missing,
/// default values are used.
#[derive(Debug, Clone)]
pub struct NeutralIpcConfig {
    /// Default host address (127.0.0.1)
    host: String,
    /// Default port number (4273)
    port: u16,
    /// Default timeout in seconds (10)
    timeout: u16,
    /// Default buffer size in bytes (8192)
    buffer_size: usize,
    /// The IPC server configuration file
    config_file: String,
}

impl Default for NeutralIpcConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 4273,
            timeout: 10,
            buffer_size: 8192,
            config_file: "/etc/neutral-ipc-cfg.json".to_string(),
        }
    }
}

impl NeutralIpcConfig {
    /// Create a new configuration with default values and load from config file if it exists
    pub fn new() -> Self {
        let mut config = Self::default();
        config.load_from_config_file();
        config
    }

    /// Load configuration from the config file and update current values
    fn load_from_config_file(&mut self) {
        let file_config = self.load_config();
        if let Value::Object(_) = file_config {
            // Override with values from config file if they exist
            if let Some(host) = file_config.get("host").and_then(|v| v.as_str()) {
                self.host = host.to_string();
            }
            if let Some(port) = file_config.get("port").and_then(|v| v.as_u64()) {
                self.port = port as u16;
            }
            if let Some(timeout) = file_config.get("timeout").and_then(|v| v.as_u64()) {
                self.timeout = timeout as u16;
            }
            if let Some(buffer_size) = file_config.get("buffer_size").and_then(|v| v.as_u64()) {
                self.buffer_size = buffer_size as usize;
            }
        }
    }

    /// Load configuration from JSON file if it exists.
    ///
    /// This method attempts to read and parse the configuration file specified
    /// in `self.config_file`. If the file doesn't exist or cannot be parsed,
    /// it returns `Value::Null`.
    fn load_config(&self) -> Value {
        if !Path::new(&self.config_file).exists() {
            return Value::Null;
        }

        match fs::read_to_string(&self.config_file) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => config,
                    Err(_) => Value::Null,
                }
            }
            Err(_) => Value::Null,
        }
    }


    /// Get configured host address
    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    /// Get configured port number
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Get configured timeout value
    pub fn get_timeout(&self) -> u16 {
        self.timeout
    }

    /// Get configured buffer size
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Get configuration file path
    pub fn get_config_file(&self) -> String {
        self.config_file.clone()
    }
    /// Set the host address
    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    /// Set the port number
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Set the timeout value
    pub fn set_timeout(&mut self, timeout: u16) {
        self.timeout = timeout;
    }

    /// Set the buffer size
    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        self.buffer_size = buffer_size;
    }

    /// Set the configuration file path
    pub fn set_config_file(&mut self, config_file: String) {
        self.config_file = config_file;
        // Automatically reload from the new config file
        self.load_from_config_file();
    }

    /// Update multiple configuration settings at once
    ///
    /// This method applies the provided settings and then automatically reloads from the config file
    /// if a config_file is specified in the settings. If no config_file is provided, it only applies
    /// the given settings without reloading from file.
    ///
    /// # Arguments
    ///
    /// * `settings` - A JSON object containing the settings to update
    ///
    /// # Example
    ///
    /// ```no_run
    /// use neutralipcrs::config::NeutralIpcConfig;
    /// use serde_json::json;
    ///
    /// let mut config = NeutralIpcConfig::new();
    /// let settings = json!({
    ///     "host": "192.168.1.1",
    ///     "port": 8080,
    ///     "timeout": 30
    /// });
    /// config.update_settings(settings);
    /// ```
    pub fn update_settings(&mut self, settings: Value) {
        // Check if config_file is being updated
        let should_reload = if let Value::Object(settings_map) = &settings {
            settings_map.get("config_file").and_then(|v| v.as_str()).is_some()
        } else {
            false
        };

        // Apply the provided settings
        if let Value::Object(settings_map) = settings {
            if let Some(host) = settings_map.get("host").and_then(|v| v.as_str()) {
                self.host = host.to_string();
            }
            if let Some(port) = settings_map.get("port").and_then(|v| v.as_u64()) {
                self.port = port as u16;
            }
            if let Some(timeout) = settings_map.get("timeout").and_then(|v| v.as_u64()) {
                self.timeout = timeout as u16;
            }
            if let Some(buffer_size) = settings_map.get("buffer_size").and_then(|v| v.as_u64()) {
                self.buffer_size = buffer_size as usize;
            }
            if let Some(config_file) = settings_map.get("config_file").and_then(|v| v.as_str()) {
                self.config_file = config_file.to_string();
            }
        }

        // Reload from config file if config_file was updated
        if should_reload {
            self.load_from_config_file();
        }
    }
}
