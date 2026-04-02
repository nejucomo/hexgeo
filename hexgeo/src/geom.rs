//! Hex geometry in [f32] cartesian space
//!
#![doc = include_str!("geom/overview.md")]
//!
//! The base geometry is defined for an equilateral hexagon inscribed in the rectangle from `(-1, -1)` to `(1, 1)`.
//!
#![doc = include_str!("geom/overview.svg")]

use emath::{Pos2, Vec2};

mod orientation;

pub use self::orientation::{DHO, HexOrientation};

/// Taken from `std` experimental nightly docs; backported for stable:
const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

/// The length of the longer of the two cartesian radii
pub const LONG_RADIUS: f32 = 1.0;

/// The length of the longer of the two cartesian diameters
pub const LONG_DIAMETER: f32 = 2.0 * LONG_RADIUS;

/// The length of the shorter of the two cartesian radii
pub const SHORT_RADIUS: f32 = SQRT_3 / 2.0;

/// The length of the shorter of the two cartesian diameters
pub const SHORT_DIAMETER: f32 = 2.0 * SHORT_RADIUS;

/// Constants for the flat-top orientation
pub mod flat_top {

    use super::*;

    // pub const BOTTOM_LEFT: Vec2 = Vec2::new(LONG_RADIUS, SHORT_RADIUS);

    /// A const definition of [HexOrientation::width_and_height]
    pub const WIDTH_AND_HEIGHT: Vec2 = Vec2::new(LONG_DIAMETER, SHORT_DIAMETER);

    /// A const definition of [HexOrientation::q_basis]
    pub const BASIS_Q: Vec2 = Vec2::new(1.5 * LONG_RADIUS, SHORT_RADIUS);

    /// A const definition of [HexOrientation::r_basis]
    pub const BASIS_R: Vec2 = Vec2::new(0.0, 2.0 * SHORT_RADIUS);

    /// A const definition of [HexOrientation::vertices]
    ///
    /// - **TODO: These were just copy-pasta and may not be precise.**
    pub const VERTICES: [Pos2; 6] = [
        Pos2::new(-1.0, 0.0),
        Pos2::new(-0.5, -0.8660254),
        Pos2::new(0.5, -0.8660254),
        Pos2::new(1.0, 0.0),
        Pos2::new(0.5, 0.8660254),
        Pos2::new(-0.5, 0.8660254),
    ];
}

/// Constants for the pointy-top orientation
pub mod pointy_top {
    use super::*;

    // pub const BOTTOM_LEFT: Vec2 = Vec2::new(SHORT_RADIUS, LONG_RADIUS);

    /// A const definition of [HexOrientation::width_and_height]
    pub const WIDTH_AND_HEIGHT: Vec2 = Vec2::new(SHORT_DIAMETER, LONG_DIAMETER);

    /// A const definition of [HexOrientation::q_basis]
    pub const BASIS_Q: Vec2 = Vec2::new(2.0 * SHORT_RADIUS, 0.0);

    /// A const definition of [HexOrientation::r_basis]
    pub const BASIS_R: Vec2 = Vec2::new(SHORT_RADIUS, 1.5 * LONG_RADIUS);

    /// A const definition of [HexOrientation::vertices]
    ///
    /// - **TODO: These were just copy-pasta and may not be precise.**
    pub const VERTICES: [Pos2; 6] = [
        Pos2::new(0.0, -1.0),
        Pos2::new(0.8660254, -0.5),
        Pos2::new(0.8660254, 0.5),
        Pos2::new(0.0, 1.0),
        Pos2::new(-0.8660254, 0.5),
        Pos2::new(-0.8660254, -0.5),
    ];
}

macro_rules! def_static_orientation {
    ( $name:ident, $cmod:ident ) => {
        /// A static definition for [HexOrientation] using the constants in
        #[doc = concat!("[", stringify!($cmod), "].")]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name;

        impl HexOrientation for $name {
            #[inline]
            fn width_and_height(self) -> Vec2 {
                $cmod::WIDTH_AND_HEIGHT
            }

            #[inline]
            fn q_basis(self) -> Vec2 {
                $cmod::BASIS_Q
            }

            #[inline]
            fn r_basis(self) -> Vec2 {
                $cmod::BASIS_R
            }

            #[inline]
            fn vertices(self) -> [Pos2; 6] {
                $cmod::VERTICES
            }
        }
    };
}

def_static_orientation!(FlatTop, flat_top);
def_static_orientation!(PointyTop, pointy_top);
