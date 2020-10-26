mod gui;
mod lib;
pub use anyhow::{anyhow, Result};
use iced::{Application, Settings};
pub use lib::Priklad;

fn main() -> Result<()> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (512, 48),
            resizable: false,
            decorations: true,
        },
        antialiasing: true,
        flags: (),
        ..Settings::default()
    };
    gui::RScalc::run(settings);
    Ok(())
}
