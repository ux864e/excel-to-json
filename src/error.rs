//! Error types for excel-to-json.

use thiserror::Error;

/// Application-level error types.
#[derive(Debug, Error)]
pub enum EError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to parse an Excel file.
    #[error("Failed to parse Excel file '{path}': {source}")]
    ExcelParse {
        path: String,
        #[source]
        source: calamine::Error,
    },

    /// Failed to serialize output to JSON.
    #[error("JSON serialization error: {0}")]
    JsonSerialize(#[from] serde_json::Error),

    /// Failed to parse TOML configuration.
    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    /// An unsupported file format was encountered.
    #[error("Unsupported file format: {path}")]
    UnsupportedFormat { path: String },

    /// A generic application error with context.
    #[error("{message}")]
    Generic { message: String },
}
