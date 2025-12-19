mod core;
mod learning;
mod misc;
mod app;
mod utils;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
