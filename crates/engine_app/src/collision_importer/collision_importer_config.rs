
#[derive(Debug, Clone, Copy)]
pub enum ResolutionMode {
    Dimensions {
        width: u16,
        height: u16,
    },
    Scale {
        scale: f32,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionImporterConfig {
    pub resolution_mode: ResolutionMode,

    // Controls how accurately we approximate the boundary.
    // Higher value = more detail.
    pub detail_level: f32,

    // Grayscale threshold (0.0 - 1.0).
    pub threshold: f32,
}
