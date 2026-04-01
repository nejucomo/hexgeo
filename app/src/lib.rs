#![deny(unsafe_code)]

mod app;
mod axext;
mod gameboard;
mod run;
mod widgets;

pub mod options;
pub use self::run::run;
