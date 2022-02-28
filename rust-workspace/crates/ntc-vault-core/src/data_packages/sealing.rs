//! Data package sealing.

use std::fs::Metadata;

use crate::data_packages::common::{DataType, SchemaType};

/// Information about how a data package was sealed.
pub struct Seal {
    // TODO
}

/// A [`DataPackage`] with a sealed dataset.
pub struct SealedDataPackage {
    pub seal: Seal,
    pub metadata: Metadata,
    pub sealed_dataset: SealedDataset,
}

/// A [`Dataset`] with sealed data.
pub struct SealedDataset {
    pub schema_type: SchemaType,
    pub schema: Box<[u8]>,

    pub data_type: DataType,
    pub sealed_data: Box<[u8]>,
}
