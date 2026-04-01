use egui::Vec2;

use self::HexOrientation::*;

/// Taken from `std` experimental nightly docs; backported for stable:
const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

const LONG_RADIUS: f32 = 0.5;
const SHORT_RADIUS: f32 = SQRT_3 / 2.0;

const BOTTOM_LEFT_FLAT: Vec2 = Vec2::new(LONG_RADIUS, SHORT_RADIUS);
const BOTTOM_LEFT_POINTY: Vec2 = Vec2::new(SHORT_RADIUS, LONG_RADIUS);

const BASIS_FLAT_Q: Vec2 = Vec2::new(1.5 * LONG_RADIUS, SHORT_RADIUS);
const BASIS_FLAT_R: Vec2 = Vec2::new(0.0, 2.0 * SHORT_RADIUS);

const BASIS_POINTY_Q: Vec2 = Vec2::new(2.0 * SHORT_RADIUS, 0.0);
const BASIS_POINTY_R: Vec2 = Vec2::new(SHORT_RADIUS, 1.5 * LONG_RADIUS);

#[derive(Copy, Clone, Debug, Default)]
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
    pub fn q_basis(self) -> Vec2 {
        match self {
            FlatTop => BASIS_FLAT_Q,
            PointyTop => BASIS_POINTY_Q,
        }
    }

    #[inline]
    pub fn r_basis(self) -> Vec2 {
        match self {
            FlatTop => BASIS_FLAT_R,
            PointyTop => BASIS_POINTY_R,
        }
    }

    /// From center to the bottom-left of a bounding rectangle
    #[inline]
    pub fn bottom_left(self) -> Vec2 {
        match self {
            FlatTop => BOTTOM_LEFT_FLAT,
            PointyTop => BOTTOM_LEFT_POINTY,
        }
    }
}
