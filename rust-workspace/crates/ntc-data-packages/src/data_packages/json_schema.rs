//! Support for JSON + JSON Schema datasets.
//!
//! # Choice of JSON Schema implementation
//!
//! (As of 2022, by Pi Delport)
//!
//! 1. <https://crates.io/crates/jsonschema>: Well-maintained, fast, comprehensive.
//! 2. <https://crates.io/crates/valico>: Inactive (~1 year), less comprehensive.
//! 3. <https://crates.io/crates/jsonschema-valid>: Inactive (~2 years), less comprehensive.

use jsonschema::JSONSchema;
use thiserror::Error;

use crate::data_packages::common::{DataType, Dataset, SchemaType};

#[derive(Debug)]
pub struct JsonDataset {
    pub schema: serde_json::Value,
    pub data: serde_json::Value,
}

impl TryFrom<Dataset> for JsonDataset {
    type Error = JsonDatasetParseError;

    fn try_from(
        Dataset {
            schema_type,
            schema,
            data_type,
            data,
        }: Dataset,
    ) -> Result<Self, Self::Error> {
        match (schema_type, data_type) {
            (SchemaType::JsonSchema, DataType::Json) => {
                let schema = serde_json::from_slice(&schema)
                    .map_err(JsonDatasetParseError::ParseSchemaFailed)?;
                let data = serde_json::from_slice(&data)
                    .map_err(JsonDatasetParseError::ParseDataFailed)?;
                Ok(Self { schema, data })
            }
        }
    }
}

impl JsonDataset {
    pub fn validate(&self) -> Result<(), JsonDatasetValidationError> {
        let compiled = JSONSchema::compile(&self.schema)
            .map_err(|err| JsonDatasetValidationError::CompileSchemaFailed(err.into()))?;
        compiled
            .validate(&self.data)
            .map_err(|errs| JsonDatasetValidationError::InvalidData(errs.into()))
    }
}

#[derive(Debug, Error)]
pub enum JsonDatasetParseError {
    #[error("failed to parse schema as JSON")]
    ParseSchemaFailed(#[source] serde_json::Error),

    #[error("failed to parse data as JSON")]
    ParseDataFailed(#[source] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum JsonDatasetValidationError {
    #[error("failed to compile JSON Schema")]
    CompileSchemaFailed(#[source] ValidationErrorMessage),

    #[error("data validation failed")]
    InvalidData(#[source] ValidationErrorMessages),
}

/// This contains the message from [`jsonschema::ValidationError`] as a string,
/// to avoid propagating the error's lifetime.
#[derive(Debug, Error)]
#[error("validation error: {0}")]
pub struct ValidationErrorMessage(String);

impl From<jsonschema::ValidationError<'_>> for ValidationErrorMessage {
    fn from(err: jsonschema::ValidationError) -> Self {
        Self(validation_error_message(err))
    }
}

/// This contains messages from [`jsonschema::ErrorIterator`] as strings,
/// to avoid propagating the iterator's lifetime.
#[derive(Debug, Error)]
#[error("validation errors: {}", .0.join(", "))]
pub struct ValidationErrorMessages(Box<[String]>);

impl From<jsonschema::ErrorIterator<'_>> for ValidationErrorMessages {
    fn from(errs: jsonschema::ErrorIterator) -> Self {
        Self(errs.map(validation_error_message).collect())
    }
}

/// Internal helper: Format a validation error as a stand-alone error message.
fn validation_error_message(err: jsonschema::ValidationError) -> String {
    format!(
        "{} (path={} schema={})",
        err, err.instance_path, err.schema_path
    )
}
