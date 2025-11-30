//! Data plane discriminants (𝒜, 𝒳, 𝒦, ℰ) as defined in software/50-data/10-axioms-and-planes.md.

/// Data planes for the Arrow spine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plane {
    Knowledge, // 𝒜
    Context,   // 𝒳
    Control,   // 𝒦
    Execution, // ℰ
}

impl Plane {
    /// Numeric tag for on-wire storage (Spine.plane).
    pub const fn tag(self) -> u8 {
        match self {
            Plane::Knowledge => 0,
            Plane::Context => 1,
            Plane::Control => 2,
            Plane::Execution => 3,
        }
    }
}
