use derive_more::{From, Into};
use emath::Pos2;

use crate::geom::HexOrientation;

/// [Axial coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-axial) for a hexagonal tiling
#[derive(Copy, Clone, Debug, From, Into)]
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

    /// The normalized cartesian center for `self` coordinates given an `orientation`
    #[inline]
    pub fn center_pos(self, orientation: impl HexOrientation) -> Pos2 {
        let Axials { q, r } = self;

        let qvec = q as f32 * orientation.q_basis();
        let rvec = r as f32 * orientation.r_basis();

        (qvec + rvec).to_pos2()
    }
}
