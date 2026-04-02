use derive_more::{From, Into};

/// A point in cartesian space with [f32] precision coordinates
#[derive(Debug, From, Into)]
pub struct Pt {
    pub x: f32,
    pub y: f32,
}

impl Pt {
    pub const ORIGIN: Pt = Pt::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Pt { x, y }
    }
}
