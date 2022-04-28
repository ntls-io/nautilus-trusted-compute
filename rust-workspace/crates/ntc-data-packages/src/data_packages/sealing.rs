//! Data package sealing.

use std::fs::Metadata;

use crate::data_packages::common::{DataType, SchemaType};

// TODO(Herman): Update doc or types here to be consistent.
//               Data packages is currently assumed to be sealed in the doc.
// TODO: Wrap unsealed representation of datasets with zeroize?

/// Information about how a data package was sealed.
pub struct Seal {
    // TODO
}

/// A [`DataPackage`] with a sealed dataset.
///
/// [`DataPackage`]: crate::data_packages::common::DataPackage
pub struct SealedDataPackage {
    pub seal: Seal,
    pub metadata: Metadata,
    pub sealed_dataset: SealedDataset,
}

/// A [`Dataset`] with sealed data.
///
/// [`Dataset`]: crate::data_packages::common::Dataset
pub struct SealedDataset {
    pub schema_type: SchemaType,
    pub schema: Box<[u8]>,

    pub data_type: DataType,
    pub sealed_data: Box<[u8]>,
}
