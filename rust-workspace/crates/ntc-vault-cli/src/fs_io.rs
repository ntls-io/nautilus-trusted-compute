//! File I/O support.

use std::fs;
use std::path::Path;

use anyhow::anyhow;
use ntc_data_packages::data_packages::common::Metadata;

enum FileType {
    Json,
}

impl FileType {
    fn for_extension(path: &Path) -> anyhow::Result<Self> {
        let extension = path
            .extension()
            .ok_or_else(|| anyhow!("file has no extension: {}", path.to_string_lossy()))?;
        if extension.eq_ignore_ascii_case("json") {
            Ok(Self::Json)
        } else {
            Err(anyhow!(
                "unsupported file extension {extension:?} ({})",
                path.to_string_lossy()
            ))
        }
    }
}

/// Read [`Metadata`] from the given file.
pub fn read_metadata(path: &Path) -> anyhow::Result<Metadata> {
    let file_type = FileType::for_extension(path)?;
    let bytes = fs::read(path)?;
    let metadata = match file_type {
        FileType::Json => Metadata::from_json_bytes(&bytes)?,
    };
    Ok(metadata)
}
