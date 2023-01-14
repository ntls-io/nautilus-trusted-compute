//! Tests for [`ntc_data_packages::data_packages::json_schema`].

use ntc_data_packages::data_packages::common::{DataType, Dataset, SchemaType};
use ntc_data_packages::data_packages::json_schema::JsonDataset;
use serde_json::{json, Value};

#[test]
fn json_try_from_no_data() {
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
fn json_try_from_empty_object() {
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
fn validate_empty() {
    let json_dataset = JsonDataset {
        schema: json!({}),
        data: json!({}),
    };
    json_dataset.validate().unwrap();
}

#[test]
fn validate_false_schema() {
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
fn validate_example_person_schema() {
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
fn validate_example_person_schema_age_missing() {
    let json_dataset = JsonDataset {
        schema: example_person_schema(),
        data: json!({
          "firstName": "Jane",
          "lastName": "Doe",
        }),
    };
    let err = json_dataset.validate().unwrap_err();
    k9::snapshot!(
        format_err(err),
        r#"
data validation failed

Caused by:
    validation errors: "age" is a required property (path= schema=/required)
"#
    );
}

#[test]
fn validate_example_person_schema_invalid() {
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
      },
      "required": [ "firstName", "lastName", "age" ],
    })
}

/// Helper: Format an error chain as a readable string.
fn format_err(err: impl Into<anyhow::Error>) -> String {
    format!("{:?}", err.into())
}
