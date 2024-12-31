// main.rs
mod ui;
mod model;
mod utilities;
use crate::ui::run;

fn main() -> color_eyre::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}