//! Tests for [`ntc_data_packages::data_packages::common`].

/// Tests for [`Metadata`].
mod metadata {
    use ntc_data_packages::data_packages::common::Metadata;
    use serde_json::json;

    #[test]
    fn from_json_bytes_empty() {
        let err = Metadata::from_json_bytes(b"").unwrap_err();
        k9::snapshot!(
            err,
            r#"Error("EOF while parsing a value", line: 1, column: 0)"#
        );
    }

    #[test]
    fn from_json_bytes_invalid() {
        let err = Metadata::from_json_bytes(b"{}").unwrap_err();
        k9::snapshot!(err, r#"Error("missing field `name`", line: 1, column: 2)"#);
    }

    #[test]
    fn from_json_bytes_valid() {
        let value = serde_json::to_vec(&json!({
            "name": "Test Data",
            "version": "0.1",
            "creator": "Test Creator",
            "timestamp": "2022-01-01",
            "description": "A test data package",
        }))
        .unwrap();
        let metadata = Metadata::from_json_bytes(&value).unwrap();
        k9::snapshot!(
            metadata,
            r#"
Metadata {
    name: "Test Data",
    version: "0.1",
    creator: "Test Creator",
    timestamp: "2022-01-01",
    description: "A test data package",
}
"#
        );
    }
}
