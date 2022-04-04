//! The common, serialised representation of data packages.

use serde::{Deserialize, Serialize};

/// A data package associates a [`Dataset`] with some descriptive [`Metadata`].
pub struct DataPackage {
    pub metadata: Metadata,
    pub dataset: Dataset,
}

/// Metadata that describes some [`Dataset`].
#[derive(Debug)] // core
#[derive(Serialize, Deserialize)] // serde
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub creator: String,
    pub timestamp: String,
    pub description: String,
}

impl Metadata {
    pub fn from_json_bytes(value: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(value)
    }
}

/// A serialised dataset contains some serialised data, and some serialised schema describing it.
#[derive(Debug)]
pub struct Dataset {
    pub schema_type: SchemaType,
    pub schema: Box<[u8]>,

    pub data_type: DataType,
    pub data: Box<[u8]>,
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum SchemaType {
    JsonSchema,
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum DataType {
    Json,
}
