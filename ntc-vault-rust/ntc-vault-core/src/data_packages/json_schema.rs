//! Support for JSON + JSON Schema datasets.
//!
//! # Choice of JSON Schema implementation
//!
//! (As of 2022, by Pi Delport)
//!
//! 1. <https://crates.io/crates/jsonschema>: Well-maintained, fast, comprehensive.
//! 2. <https://crates.io/crates/valico>: Inactive (~1 year), less comprehensive.
//! 3. <https://crates.io/crates/jsonschema-valid>: Inactive (~2 years), less comprehensive.

use anyhow::anyhow;
use jsonschema::JSONSchema;

use crate::data_packages::common::{DataType, Dataset, SchemaType};

#[derive(Debug)]
pub struct JsonDataset {
    pub schema: serde_json::Value,
    pub data: serde_json::Value,
}

impl TryFrom<Dataset> for JsonDataset {
    type Error = anyhow::Error;

    fn try_from(
        Dataset {
            schema_type,
            schema,
            data_type,
            data,
        }: Dataset,
    ) -> Result<Self, Self::Error> {
        match (schema_type, data_type) {
            (SchemaType::JsonSchema, DataType::Json) => Ok(Self {
                schema: serde_json::from_slice(schema.as_ref())?,
                data: serde_json::from_slice(data.as_ref())?,
            }),
        }
    }
}

impl JsonDataset {
    pub fn validate(&self) -> anyhow::Result<()> {
        let compiled = JSONSchema::compile(&self.schema)
            // XXX: Convert validation error to a string, to avoid lifetime leaking out.
            .map_err(|err| anyhow!("Compiling schema failed").context(err.to_string()))?;
        compiled
            .validate(&self.data)
            // Convert validation errors into a single error.
            .map_err(move |errs| {
                let messages: String = errs.map(move |err| format!("{}\n", err)).collect();
                anyhow!("Validation failed").context(messages)
            })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

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
        assert_eq!(
            err.to_string(),
            "EOF while parsing a value at line 1 column 0"
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
        assert_eq!(err.to_string(), "False schema does not allow {}\n");
    }
}
