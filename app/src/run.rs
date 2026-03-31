use clap::Parser as _;
use color_eyre::eyre::{Result, eyre};
use env_logger::Logger;
use logging_options::Backend as _;

use crate::app::App;
use crate::options::Options;

/// Run the app
///
/// # Caution
///
/// This expects to control all global process initialization/cleanup in the same manner as a `main` function. It is written expecting basically a `fn main() -> Result { run() }` wrapper. The means it does things like initializing global logging, managing uncaught error reporting, etc...
pub fn run() -> Result<()> {
    color_eyre::install()?;

    let opts = Options::parse();
    init_log(&opts.logopts);

    App::run().map_err(|e| eyre!("{e}"))?;

    Ok(())
}

fn init_log(logopts: &logging_options::StandardConsole) {
    use logging_options::backend::LoggingOptions as _;

    let mut b = Logger::builder();

    for noisymod in ["eframe", "egui", "egui_glow", "egui_winit"] {
        b.filter_module(noisymod, log::LevelFilter::Info);
    }

    logopts.configure(b).init();

    log::debug!("Logging initialized.");
}
