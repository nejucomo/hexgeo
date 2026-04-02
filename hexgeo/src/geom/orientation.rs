use std::fmt::Debug;

use emath::{Pos2, Rect, Vec2};

use crate::geom::{FlatTop, PointyTop};

/// Types which represent the hex orientations
///
/// All coordinates are for a hexagon centered at the origin, scaled to fix into the `(-1, -1)` to `(1, 1)` square. See [geom](crate::geom) for visualizations of the layout.
pub trait HexOrientation: Copy + Clone + Debug + Default + Eq + PartialEq {
    /// The width and height of the bounding rectangle centered at the origin
    fn width_and_height(self) -> Vec2;

    /// The q-basis vector in the axial coordinate system; see [Hex to Pixel: Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial)
    fn q_basis(self) -> Vec2;

    /// The r-basis vector in the axial coordinate system; see [Hex to Pixel: Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial)
    fn r_basis(self) -> Vec2;

    /// The six vertices of the hex.
    fn vertices(self) -> [Pos2; 6];

    /// The bounding rect of a single hex centered at the origin
    #[inline]
    fn bounding_rect(self) -> Rect {
        Rect::from_center_size(Pos2::ZERO, self.width_and_height())
    }

    /// The bounding rectangle for a disc of given `radius`
    #[inline]
    fn disc_bounding_rect(self, radius: usize) -> Rect {
        let frad = radius as f32;
        let Pos2 {
            x: right,
            y: bottom,
        } = self.bounding_rect().right_bottom();
        let width = self.q_basis().x * frad + right;
        let height = self.r_basis().y * frad + bottom;

        Rect::from_x_y_ranges(-width..=width, -height..=height)
    }
}

/// <u>D</u>ynamic <u>H</u>ex <u>O</u>rientation is a runtime switch on [HexOrientation]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum DHO {
    /// The flat top orientation
    #[default]
    FlatTop,
    /// The pointy top orientation
    PointyTop,
}

impl HexOrientation for DHO {
    #[inline]
    fn width_and_height(self) -> Vec2 {
        match self {
            DHO::FlatTop => FlatTop.width_and_height(),
            DHO::PointyTop => PointyTop.width_and_height(),
        }
    }

    #[inline]
    fn q_basis(self) -> Vec2 {
        match self {
            DHO::FlatTop => FlatTop.q_basis(),
            DHO::PointyTop => PointyTop.q_basis(),
        }
    }

    #[inline]
    fn r_basis(self) -> Vec2 {
        match self {
            DHO::FlatTop => FlatTop.r_basis(),
            DHO::PointyTop => PointyTop.r_basis(),
        }
    }

    #[inline]
    fn vertices(self) -> [Pos2; 6] {
        match self {
            DHO::FlatTop => FlatTop.vertices(),
            DHO::PointyTop => PointyTop.vertices(),
        }
    }
}
