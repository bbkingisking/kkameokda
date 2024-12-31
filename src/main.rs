// main.rs
mod ui;
mod model;
mod utilities;
mod load;

use crate::ui::run;
use crate::load::load_decks;
use crate::utilities::print_deck_structure;

fn main() -> color_eyre::Result<()> {
    let decks = load_decks()?;
    println!("Loaded {} decks:", decks.len());
    
    for deck in &decks {
        print_deck_structure(deck, 0);
    }
    
    let terminal = ratatui::init();
    let result = run(terminal, decks);
    ratatui::restore();
    result
}
