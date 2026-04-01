use egui::{Pos2, Rect};
use extension_traits::extension;
use hexohexes::{AxialBounds, Axials};

use crate::HexOrientation;

/// An extension trait to compute (normalized-scale) pixel coordinates to/from an [Axials]
///
/// # Scaling
///
/// This uses a normalized scale with the long diameter of a hex equal to 1.0. Thus a hex fits into a 1.0 x 1.0 square, and that at `Axials::CENTER` fits into the rectangle `(-0.5, -0.5)` to `(0.5, 0.5)` with center at `(0, 0)`. In the [HexOrientation::FlatTop] the center hex intersects the square at exactly the two points on the X-axis. In the [HexOrientation::PointyTop] the hex intersects at exactly the two points on the Y-axis.
#[extension(pub trait AxialsExt)]
impl Axials {
    /// The center [Pos2] of `self` where `Axials::CENTER.center_pos(…) == Pos2::ZERO`
    fn center_pos(self, orientation: HexOrientation) -> Pos2 {
        let Axials { q, r } = self;

        let qvec = q as f32 * orientation.q_basis();
        let rvec = r as f32 * orientation.r_basis();

        (qvec + rvec).to_pos2()
    }
}

/// An extension trait to providing (normalized-scale) pixel values for [AxialBounds]
///
#[extension(pub trait AxialBoundsExt)]
impl AxialBounds {
    /// The bounding [Rect] such that all hexes fix entirely within this region.
    ///
    /// # Scaling
    ///
    /// This uses the same scaling as described in the [AxialsExt] trait docs.
    fn bounding_rect(&self, orientation: HexOrientation) -> Rect {
        let bl = orientation.bottom_left();
        let frad = self.radius() as f32;
        let width = orientation.q_basis().x * frad + bl.x;
        let height = orientation.r_basis().y * frad + bl.y;

        Rect::from_x_y_ranges(-width..=width, -height..=height)
    }
}
