//! `hexgeo` - <u>hex</u>agonal <u>geo</u>metry and tiling utilities
//!
//! This crate, similar to many hexagon graphics/games trait relies heavily on the superb [Hexagonal Grids from Red Blob Games](https://www.redblobgames.com/grids/hexagons/).
//!
#![doc = include_str!("geom/overview.md")]
//!
//! See [geom] for more detail.
//!
#![doc = include_str!("geom/overview.svg")]
//!
//! # TODO / Messy Notes
//!
//! - define `disc` or use the term from RedBlob.
//! - back-out `RadialIndexMap` for a stateless impl for API/impl simplicity. The stateful version needs to be justified w/ profiling.
#![deny(unsafe_code, missing_docs)]

mod axials;

pub mod geom;
pub mod radial;
pub use self::axials::Axials;
