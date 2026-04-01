#![deny(unsafe_code)]

mod app;
mod gameboard;
mod run;

pub mod options;
pub use self::run::run;
