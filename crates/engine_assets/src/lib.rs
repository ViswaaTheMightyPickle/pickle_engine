use std::path::Path;

use engine_core::{ImportedObject, MaterialId};

#[derive(Clone, Copy, Debug)]
pub struct PaletteEntry {
    pub rgba: [u8; 4],
    pub material: MaterialId,
}

pub fn import_object(
    _sprite: &Path,
    _action_mask: &Path,
    _collision_mask: &Path,
    _palette: &[PaletteEntry],
) -> Result<ImportedObject, String> {
    // TODO (Gienna): decode PNGs with image crate, validate dimensions,
    // map action RGB + alpha, validate binary collision mask.
    Err("importer is not implemented yet".to_string())
}
