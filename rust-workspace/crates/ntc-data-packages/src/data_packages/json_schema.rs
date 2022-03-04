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

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn test_json_try_from_no_data() {
        let dataset = Dataset {
            schema_type: SchemaType::JsonSchema,
            schema: "".as_bytes().into(),
            data_type: DataType::Json,
            data: "".as_bytes().into(),
        };
        let err = JsonDataset::try_from(dataset).unwrap_err();
        k9::snapshot!(
            format_err(err),
            "
failed to parse schema as JSON

Caused by:
    EOF while parsing a value at line 1 column 0
"
        );
    }

    #[test]
    fn test_json_try_from_empty_object() {
        let dataset = Dataset {
            schema_type: SchemaType::JsonSchema,
            schema: "{}".as_bytes().into(),
            data_type: DataType::Json,
            data: "{}".as_bytes().into(),
        };
        let json_dataset = JsonDataset::try_from(dataset).unwrap();
        assert_eq!(json_dataset.schema, json!({}));
        assert_eq!(json_dataset.data, json!({}));
    }

    #[test]
    fn test_validate_empty() {
        let json_dataset = JsonDataset {
            schema: json!({}),
            data: json!({}),
        };
        json_dataset.validate().unwrap();
    }

    #[test]
    fn test_validate_false_schema() {
        let json_dataset = JsonDataset {
            schema: json!(false),
            data: json!({}),
        };
        let err = json_dataset.validate().unwrap_err();
        k9::snapshot!(
            format_err(err),
            "
data validation failed

Caused by:
    validation errors: False schema does not allow {} (path= schema=)
"
        );
    }

    #[test]
    fn test_validate_example_person_schema() {
        let json_dataset = JsonDataset {
            schema: example_person_schema(),
            data: json!({
              "firstName": "John",
              "lastName": "Doe",
              "age": 21
            }),
        };
        json_dataset.validate().expect("validate should succeed");
    }

    #[test]
    fn test_validate_example_person_schema_invalid() {
        let json_dataset = JsonDataset {
            schema: example_person_schema(),
            data: json!({
                "firstName": false,
                "lastName": null,
                "age": -1,
            }),
        };
        let err = json_dataset.validate().unwrap_err();
        k9::snapshot!(
            format_err(err),
            r#"
data validation failed

Caused by:
    validation errors: -1 is less than the minimum of 0 (path=/age schema=/properties/age/minimum), false is not of type "string" (path=/firstName schema=/properties/firstName/type), null is not of type "string" (path=/lastName schema=/properties/lastName/type)
"#
        );
    }

    /// The `person.schema.json` example schema from <https://json-schema.org/learn/miscellaneous-examples.html>.
    fn example_person_schema() -> Value {
        json!({
          "$id": "https://example.com/person.schema.json",
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "title": "Person",
          "type": "object",
          "properties": {
            "firstName": {
              "type": "string",
              "description": "The person's first name."
            },
            "lastName": {
              "type": "string",
              "description": "The person's last name."
            },
            "age": {
              "description": "Age in years which must be equal to or greater than zero.",
              "type": "integer",
              "minimum": 0
            }
          }
        })
    }

    /// Helper: Format an error chain as a readable string.
    fn format_err(err: impl Into<anyhow::Error>) -> String {
        format!("{:?}", err.into())
    }
}
