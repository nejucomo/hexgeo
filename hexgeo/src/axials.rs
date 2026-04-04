use derive_more::{From, Into};
use emath::Vec2;

use crate::geom::HexOrientation;

/// [Axial coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-axial) for a hexagonal tiling
#[derive(Copy, Clone, Debug, From, Into, Eq, PartialEq, Ord, PartialOrd)]
pub struct Axials {
    /// The `q` axis coordinate
    pub q: isize,
    /// The `r` axis coordinate
    pub r: isize,
}

impl Axials {
    /// The (0, 0) hex coordinate
    pub const ORIGIN: Axials = Axials::new(0, 0);

    /// Construct new [Axials] from component coordinates
    #[inline]
    pub const fn new(q: isize, r: isize) -> Self {
        Axials { q, r }
    }

    /// A vector from normalized cartesian origin to the center of `self` coordinates given an `orientation`
    #[inline]
    pub fn origin_to_center(self, orientation: impl HexOrientation) -> Vec2 {
        let Axials { q, r } = self;

        let qvec = q as f32 * orientation.q_basis();
        let rvec = r as f32 * orientation.r_basis();

        qvec + rvec
    }
}
