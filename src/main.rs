// main.rs
mod ui;
mod model;
mod utilities;
mod load;

use crate::ui::run;
use crate::load::load_decks;

fn main() -> color_eyre::Result<()> {
    let decks = load_decks()?;
    println!("Loaded {} decks:", decks.len());
    
    for deck in &decks {
        println!("- {} ({} cards)", deck.name, deck.cards.len());
    }
    
    let terminal = ratatui::init();
    let result = run(terminal, decks);
    ratatui::restore();
    result
}