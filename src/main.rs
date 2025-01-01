// main.rs
mod ui;
mod model;
mod utilities;
mod load;
mod app;
mod app_runner;
mod args;

use crate::app_runner::run;
use crate::load::load_decks;
use crate::utilities::print_deck_structure;
use crate::utilities::print_session_summary;

fn main() -> color_eyre::Result<()> {
    let decks = load_decks()?;
    println!("Loaded {} decks:", decks.len());
    
    for deck in &decks {
        print_deck_structure(deck, 0);
    }
    
    let terminal = ratatui::init();
    let result = run(terminal, decks);
    ratatui::restore();

    if let Ok(app) = &result {
        print_session_summary(app.remembered_count, app.forgotten_count);
    }

    result.map(|_| ())
}
