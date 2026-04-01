//! Hex geometry in [f32] cartesian space
//!
//! The base geometry is defined for an equilateral hexagon inscribed in the rectangle from `(-1, -1)` to `(1, 1)`. There are two supported orientations, defined by the symmetrically named constants in [flat_top] and [pointy_top].
//!
//! # Flat Top Orientation
//!
#![doc = include_str!("geom/flat_top.svg")]
mod orientation;
mod pt;

pub use self::orientation::HexOrientation;
pub use self::pt::Pt;

/// Taken from `std` experimental nightly docs; backported for stable:
const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

pub const LONG_RADIUS: f32 = 0.5;
pub const SHORT_RADIUS: f32 = SQRT_3 / 2.0;

/// Constants for the flat-top orientation
pub mod flat_top {
    use super::*;

    pub const BOTTOM_LEFT: Pt = Pt::new(LONG_RADIUS, SHORT_RADIUS);
    pub const WIDTH_AND_HEIGHT: (f32, f32) = (LONG_RADIUS, SHORT_RADIUS);
    pub const BASIS_Q: Pt = Pt::new(1.5 * LONG_RADIUS, SHORT_RADIUS);
    pub const BASIS_R: Pt = Pt::new(0.0, 2.0 * SHORT_RADIUS);
}

pub mod pointy_top {
    use super::*;

    pub const BOTTOM_LEFT: Pt = Pt::new(SHORT_RADIUS, LONG_RADIUS);
    pub const WIDTH_AND_HEIGHT: (f32, f32) = (SHORT_RADIUS, LONG_RADIUS);
    pub const BASIS_Q: Pt = Pt::new(2.0 * SHORT_RADIUS, 0.0);
    pub const BASIS_R: Pt = Pt::new(SHORT_RADIUS, 1.5 * LONG_RADIUS);
}
