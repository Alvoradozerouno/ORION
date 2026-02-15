//! Deterministic temperature cap enforcement

/// Enforces temperature <= max. Returns clamped value.
pub struct TemperatureCap {
    pub max: f32,
}

impl TemperatureCap {
    pub fn new(max: f32) -> Self {
        Self { max }
    }

    pub fn enforce(&self, requested: f32) -> f32 {
        if requested <= self.max {
            requested
        } else {
            self.max
        }
    }
}
