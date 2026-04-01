use crate::geom::{LONG_RADIUS, Pt, SHORT_RADIUS, flat_top, pointy_top};

use self::HexOrientation::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HexOrientation {
    #[default]
    FlatTop,
    PointyTop,
}

impl HexOrientation {
    #[inline]
    pub fn width_and_height(self) -> (f32, f32) {
        match self {
            FlatTop => (LONG_RADIUS, SHORT_RADIUS),
            PointyTop => (SHORT_RADIUS, LONG_RADIUS),
        }
    }

    #[inline]
    pub fn q_basis(self) -> Pt {
        match self {
            FlatTop => flat_top::BASIS_Q,
            PointyTop => pointy_top::BASIS_Q,
        }
    }

    #[inline]
    pub fn r_basis(self) -> Pt {
        match self {
            FlatTop => flat_top::BASIS_R,
            PointyTop => pointy_top::BASIS_R,
        }
    }

    /// From center to the bottom-left of a bounding rectangle
    #[inline]
    pub fn bottom_left(self) -> Pt {
        match self {
            FlatTop => flat_top::BOTTOM_LEFT,
            PointyTop => pointy_top::BOTTOM_LEFT,
        }
    }

    #[inline]
    pub fn vertices(self) -> [Pt; 6] {
        match self {
            FlatTop => todo!(),
            PointyTop => todo!(),
        }
    }
}
