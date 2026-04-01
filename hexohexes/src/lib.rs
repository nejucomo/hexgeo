//! Hex of hexes for hexagon-tiled games
#![deny(unsafe_code)]

mod axials;
mod board;
mod bounds;

pub use self::axials::Axials;
pub use self::board::Board;
pub use self::bounds::AxialBounds;
