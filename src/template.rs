//! High-level template processing interface.
//!
//! This module provides the main user-facing API for template processing
//! through the Neutral IPC server. It handles template setup, schema management,
//! and result processing.

use serde_json::Value;
use std::collections::HashMap;
use crate::client::NeutralIpcClient;
use crate::constants::*;
use crate::error::{NeutralIpcError, Result};

/// Main interface for template processing through the Neutral IPC server.
///
/// This struct provides a high-level API for:
/// - Template setup (from file path or source code)
/// - JSON schema management and merging
/// - Template rendering via IPC communication
/// - Result processing and error handling
///
/// # Examples
///
/// ```no_run
/// use neutralipcrs::{NeutralIpcTemplate, NeutralIpcConfig};
/// use serde_json::json;
///
/// // Create template from source code
/// let schema = json!({
///     "data": {
///         "text": "World"
///     }
/// });
/// let mut template = NeutralIpcTemplate::from_src_value("Hello {:;text:}!", schema).unwrap();
/// let result = template.render().unwrap();
/// println!("{}", result); // Output: "Hello World!"
/// ```
pub struct NeutralIpcTemplate {
    /// Template content or file path
    template: String,
    /// Content type identifier (CONTENT_PATH or CONTENT_TEXT)
    tpl_type: u8,
    /// JSON schema as a string
    schema: String,
    /// Parsed result from the last rendering operation
    pub(crate) result: HashMap<String, Value>,
}

impl NeutralIpcTemplate {
    /// Create a new template instance with default settings.
    ///
    /// The template is initialized with:
    /// - Empty template content
    /// - File-based template type (CONTENT_PATH)
    /// - Empty JSON schema ("{}")
    /// - Empty result map
    ///
    /// # Returns
    ///
    /// A new `NeutralIpcTemplate` instance or an error if initialization fails.
    pub fn new() -> Result<Self> {
        Ok(Self {
            template: "".to_string(),
            tpl_type: CONTENT_PATH,
            schema: "{}".to_string(),
            result: HashMap::new(),
        })
    }

    /// Create a template from a file path and JSON schema.
    ///
    /// # Arguments
    ///
    /// * `template` - File path to the template
    /// * `schema` - JSON schema as a `Value` or string
    ///
    /// # Returns
    ///
    /// A new `NeutralIpcTemplate` instance configured for file-based processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be serialized to JSON.
    pub fn from_file_value(template: &str, schema: Value) -> Result<Self> {
        let schema_str = if schema.is_string() {
            schema.as_str().unwrap().to_string()
        } else {
            serde_json::to_string(&schema)?
        };

        Ok(Self {
            template: template.to_string(),
            tpl_type: CONTENT_PATH,
            schema: schema_str,
            result: HashMap::new(),
        })
    }

    /// Create a template from source code and JSON schema.
    ///
    /// # Arguments
    ///
    /// * `template` - Template source code as a string
    /// * `schema` - JSON schema as a `Value` or string
    ///
    /// # Returns
    ///
    /// A new `NeutralIpcTemplate` instance configured for source-based processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be serialized to JSON.
    pub fn from_src_value(template: &str, schema: Value) -> Result<Self> {
        let schema_str = if schema.is_string() {
            schema.as_str().unwrap().to_string()
        } else {
            serde_json::to_string(&schema).unwrap()
        };

        Ok(Self {
            template: template.to_string(),
            tpl_type: CONTENT_TEXT,
            schema: schema_str,
            result: HashMap::new(),
        })
    }


    /// Render the template with the current schema through the Neutral server.
    ///
    /// This method:
    /// 1. Creates an IPC client with the current template and schema
    /// 2. Sends the request to the Neutral server
    /// 3. Processes the response and extracts the rendered content
    /// 4. Stores the complete result for later inspection
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - IPC communication with the server fails
    /// - The server returns an invalid response
    /// - The response cannot be parsed as JSON
    ///
    /// # Example
    ///
    /// ```no_run
    /// use neutralipcrs::NeutralIpcTemplate;
    /// use serde_json::json;
    ///
    /// let schema = json!({"user": {"name": "Alice"}});
    /// let mut template = NeutralIpcTemplate::from_src_value("Hello {:;user.name:}!", schema).unwrap();
    /// let result = template.render().unwrap();
    /// assert_eq!(result, "Hello Alice!");
    /// ```
    pub fn render(&mut self) -> Result<String> {
        let mut client = NeutralIpcClient::new(
            CTRL_PARSE_TEMPLATE,
            CONTENT_JSON,
            &self.schema,
            self.tpl_type,
            &self.template
        );

        let result = client.start()?;

        let status = result.get("control")
            .and_then(|v| v.as_u64())
            .ok_or(NeutralIpcError::InvalidResponse)? as u8;

        let content1 = result.get("content-1")
            .and_then(|v| v.as_str())
            .ok_or(NeutralIpcError::InvalidResponse)?;

        let content2 = result.get("content-2")
            .and_then(|v| v.as_str())
            .ok_or(NeutralIpcError::InvalidResponse)?;

        let result_data: Value = serde_json::from_str(content1)?;
        self.result = HashMap::new();
        self.result.insert("status".to_string(), Value::Number(status.into()));
        self.result.insert("result".to_string(), result_data);
        self.result.insert("content".to_string(), Value::String(content2.to_string()));

        Ok(content2.to_string())
    }

    /// Set the template to use a file path.
    ///
    /// Changes the template type to `CONTENT_PATH` and updates the template content
    /// to the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the template file
    pub fn set_path(&mut self, path: &str) {
        self.tpl_type = CONTENT_PATH;
        self.template = path.to_string();
    }

    /// Set the template to use source code directly.
    ///
    /// Changes the template type to `CONTENT_TEXT` and updates the template content
    /// to the provided source code string.
    ///
    /// # Arguments
    ///
    /// * `source` - Template source code
    pub fn set_source(&mut self, source: &str) {
        self.tpl_type = CONTENT_TEXT;
        self.template = source.to_string();
    }

    /// Merge new schema data with the existing schema.
    ///
    /// This method performs a deep merge of JSON objects, allowing you to
    /// incrementally build up complex schemas. For objects with overlapping
    /// keys, the new values will override the existing ones.
    ///
    /// # Arguments
    ///
    /// * `schema` - New schema data to merge (as `Value` or string)
    ///
    /// # Returns
    ///
    /// `Ok(())` if the merge was successful, or an error if schema parsing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use neutralipcrs::NeutralIpcTemplate;
    /// use serde_json::json;
    ///
    /// let mut template = NeutralIpcTemplate::new().unwrap();
    /// template.merge_schema(json!({"base": {"value": 1}})).unwrap();
    /// template.merge_schema(json!({"base": {"extra": 2}})).unwrap();
    /// // Schema now contains: {"base": {"value": 1, "extra": 2}}
    /// ```
    pub fn merge_schema(&mut self, schema: Value) -> Result<()> {
        let current_schema: Value = serde_json::from_str(&self.schema)?;
        let new_schema = if schema.is_string() {
            serde_json::from_str(schema.as_str().unwrap())?
        } else {
            schema
        };

        let merged = Self::deep_merge(current_schema, new_schema);
        self.schema = serde_json::to_string(&merged)?;
        Ok(())
    }

    /// Check if the last rendering operation resulted in an error.
    ///
    /// This method examines the result from the last `render()` call and
    /// determines if an error occurred based on:
    /// - The status code (non-zero indicates error)
    /// - The `has_error` field in the result data
    ///
    /// # Returns
    ///
    /// `true` if an error occurred, `false` otherwise.
    pub fn has_error(&self) -> bool {
        if let Some(status) = self.result.get("status").and_then(|v| v.as_u64()) {
            if status != 0 {
                return true;
            }
        }

        if let Some(result) = self.result.get("result") {
            if let Some(has_error) = result.get("has_error").and_then(|v| v.as_bool()) {
                return has_error;
            }
        }

        false
    }

    /// Get the status code from the last rendering result.
    ///
    /// # Returns
    ///
    /// The status code as `i64` from the JSON, or `0` if not present or if
    /// any error occurs during extraction.
    pub fn get_status_code(&self) -> &str {
        self.result.get("result")
            .and_then(|r| r.get("status_code"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }

    /// Get the status text from the last rendering result.
    ///
    /// # Returns
    ///
    /// The status text as `&str` from the JSON, or an empty string reference
    /// if not present or if any error occurs during extraction.
    pub fn get_status_text(&self) -> &str {
        self.result.get("result")
            .and_then(|r| r.get("status_text"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }

    /// Get the status parameter from the last rendering result.
    ///
    /// # Returns
    ///
    /// The status parameter as `&str` from the JSON, or an empty string reference
    /// if not present or if any error occurs during extraction.
    pub fn get_status_param(&self) -> &str {
        self.result.get("result")
            .and_then(|r| r.get("status_param"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }

    /// Get the complete result data from the last rendering operation.
    ///
    /// # Returns
    ///
    /// A reference to the complete result `Value` if available, or `None`
    /// if no result has been stored yet.
    pub fn get_result(&self) -> Option<&Value> {
        self.result.get("result")
    }

    /// Recursively merge two JSON values.
    ///
    /// For objects, this performs a deep merge where fields from `b` override
    /// or are added to fields in `a`. For all other types, `b` completely
    /// replaces `a`.
    ///
    /// # Arguments
    ///
    /// * `a` - The base JSON value
    /// * `b` - The JSON value to merge into `a`
    ///
    /// # Returns
    ///
    /// The merged JSON value.
    fn deep_merge(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Object(mut map_a), Value::Object(map_b)) => {
                for (key, value_b) in map_b {
                    if let Some(value_a) = map_a.get_mut(&key) {
                        *value_a = Self::deep_merge(value_a.clone(), value_b);
                    } else {
                        map_a.insert(key, value_b);
                    }
                }
                Value::Object(map_a)
            }
            (_, b) => b,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::client::is_server_available;

    /// Skip test if the Neutral server is not available.
    ///
    /// This helper function checks server availability and panics with a
    /// clear message if the server is not running, allowing tests to be
    /// skipped gracefully during development.
    fn skip_if_server_unavailable() {
        if !is_server_available() {
            panic!("Neutral TS server not available - skipping test");
        }
    }

    #[test]
    fn test_template_src() {
        skip_if_server_unavailable();

        let schema = json!({
            "data": {
                "text": "Hello!",
                "number": 123
            }
        });

        let mut template = NeutralIpcTemplate::from_src_value("Rust IPC client: {:;text:} {:;number:}", schema).unwrap();
        let result = template.render().unwrap();
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        assert!(!template.has_error());
        assert_eq!(status_code, "200");
        assert_eq!(status_text, "OK");
        assert_eq!(status_param, "");
        assert_eq!(result, "Rust IPC client: Hello! 123");
    }

    #[test]
    fn test_template_file() {
        skip_if_server_unavailable();

        let schema = json!({
            "data": {
                "text": "Hello!",
                "number": 123
            }
        });

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let tpl_file = format!("{}/tests/template.ntpl", manifest_dir);

        let mut template = NeutralIpcTemplate::from_file_value(&tpl_file, schema).unwrap();
        let result = template.render().unwrap();
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        assert!(!template.has_error());
        assert_eq!(status_code, "200");
        assert_eq!(status_text, "OK");
        assert_eq!(status_param, "");
        assert_eq!(result, "Rust IPC client: Hello! 123");
    }

    #[test]
    fn test_template_404() {
        skip_if_server_unavailable();

        let schema = json!({
            "data": {
                "text": "Hello!",
                "number": 123
            }
        });

        let mut template = NeutralIpcTemplate::from_src_value("Rust IPC client: {:exit; 404 :}", schema).unwrap();
        let result = template.render().unwrap();
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        assert!(!template.has_error());
        assert_eq!(status_code, "404");
        assert_eq!(status_text, "Not Found");
        assert_eq!(status_param, "");
        assert_eq!(result, "404 Not Found");
    }

    #[test]
    fn test_template_redirect() {
        skip_if_server_unavailable();

        let schema = json!({
            "data": {
                "text": "Hello!",
                "number": 123
            }
        });

        let mut template = NeutralIpcTemplate::from_src_value("Rust IPC client: {:redirect; 301 >> https://crates.io/crates/neutralts :}", schema).unwrap();
        let result = template.render().unwrap();
        let status_code = template.get_status_code();
        let status_text = template.get_status_text();
        let status_param = template.get_status_param();

        assert!(!template.has_error());
        assert_eq!(status_code, "301");
        assert_eq!(status_text, "Moved Permanently");
        assert_eq!(status_param, "https://crates.io/crates/neutralts");
        assert_eq!(result, "301 Moved Permanently\nhttps://crates.io/crates/neutralts");
    }

}
