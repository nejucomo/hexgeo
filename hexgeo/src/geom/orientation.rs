use std::fmt::Debug;

use crate::geom::{FlatTop, PointyTop, Pt};

/// Types which represent the hex orientations
///
/// All coordinates are for a hexagon centered at the origin, scaled to fix into the `(-1, -1)` to `(1, 1)` square. See [geom](crate::geom) for visualizations of the layout.
pub trait HexOrientation: Copy + Clone + Debug + Default + Eq + PartialEq {
    /// The width and height of the bounding rectangle centered at the origin
    fn width_and_height(self) -> (f32, f32);

    /// The q-basis vector in the axial coordinate system; see [Hex to Pixel: Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial)
    fn q_basis(self) -> Pt;

    /// The r-basis vector in the axial coordinate system; see [Hex to Pixel: Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial)
    fn r_basis(self) -> Pt;

    /// The six vertices of the hex.
    fn vertices(self) -> [Pt; 6];
}

/// <u>D</u>ynamic <u>H</u>ex <u>O</u>rientation is a runtime switch on [HexOrientation]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum DHO {
    #[default]
    FlatTop,
    PointyTop,
}

impl HexOrientation for DHO {
    #[inline]
    fn width_and_height(self) -> (f32, f32) {
        match self {
            DHO::FlatTop => FlatTop.width_and_height(),
            DHO::PointyTop => PointyTop.width_and_height(),
        }
    }

    #[inline]
    fn q_basis(self) -> Pt {
        match self {
            DHO::FlatTop => FlatTop.q_basis(),
            DHO::PointyTop => PointyTop.q_basis(),
        }
    }

    #[inline]
    fn r_basis(self) -> Pt {
        match self {
            DHO::FlatTop => FlatTop.r_basis(),
            DHO::PointyTop => PointyTop.r_basis(),
        }
    }

    #[inline]
    fn vertices(self) -> [Pt; 6] {
        match self {
            DHO::FlatTop => FlatTop.vertices(),
            DHO::PointyTop => PointyTop.vertices(),
        }
    }
}
