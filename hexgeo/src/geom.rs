//! Hex geometry in [f32] cartesian space
//!
#![doc = include_str!("geom/overview.md")]
//!
//! The base geometry is defined for an equilateral hexagon inscribed in the rectangle from `(-1, -1)` to `(1, 1)`.
//!
#![doc = include_str!("geom/overview.svg")]

mod orientation;
mod pt;

pub use self::orientation::{DHO, HexOrientation};
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
    // TODO: These were just copy-pasta and may not be precise
    pub const VERTICES: [Pt; 6] = [
        Pt::new(0.0, -1.0),
        Pt::new(0.8660254, -0.5),
        Pt::new(0.8660254, 0.5),
        Pt::new(0.0, 1.0),
        Pt::new(-0.8660254, 0.5),
        Pt::new(-0.8660254, -0.5),
    ];
}

/// Constants for the pointy-top orientation
pub mod pointy_top {
    use super::*;

    pub const BOTTOM_LEFT: Pt = Pt::new(SHORT_RADIUS, LONG_RADIUS);
    pub const WIDTH_AND_HEIGHT: (f32, f32) = (SHORT_RADIUS, LONG_RADIUS);
    pub const BASIS_Q: Pt = Pt::new(2.0 * SHORT_RADIUS, 0.0);
    pub const BASIS_R: Pt = Pt::new(SHORT_RADIUS, 1.5 * LONG_RADIUS);
    pub const VERTICES: [Pt; 6] = [
        Pt::new(-1.0, 0.0),
        Pt::new(-0.5, -0.8660254),
        Pt::new(0.5, -0.8660254),
        Pt::new(1.0, 0.0),
        Pt::new(0.5, 0.8660254),
        Pt::new(-0.5, 0.8660254),
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
            fn width_and_height(self) -> (f32, f32) {
                $cmod::WIDTH_AND_HEIGHT
            }

            #[inline]
            fn q_basis(self) -> Pt {
                $cmod::BASIS_Q
            }

            #[inline]
            fn r_basis(self) -> Pt {
                $cmod::BASIS_R
            }

            #[inline]
            fn vertices(self) -> [Pt; 6] {
                $cmod::VERTICES
            }
        }
    };
}

def_static_orientation!(FlatTop, flat_top);
def_static_orientation!(PointyTop, pointy_top);
