// app_runner.rs
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    DefaultTerminal,
};
use crate::model::{Card};
use crate::model::Deck;
use crate::app::App;
use color_eyre::Result;

pub fn collect_due_cards<'a>(decks: &'a [Deck], current_time: u64) -> Vec<(&'a Card, &'a str)> {
    let mut cards = Vec::new();
    
    for deck in decks {
        // Add cards from current deck
        cards.extend(
            deck.cards.iter()
                .filter(|card| card.next_review < Some(current_time))
                .map(|card| (card, deck.name.as_str()))
        );
        
        // Recursively add cards from subdecks
        if let Some(subdecks) = &deck.subdecks {
            cards.extend(collect_due_cards(subdecks, current_time));
        }
    }
    
    cards
}

pub fn run(mut terminal: DefaultTerminal, decks: Vec<Deck>) -> Result<()> {
    let mut app = App::new(decks);

    if app.due_cards_count() == 0 {
        return Err(color_eyre::eyre::eyre!("No cards due for review"));
    }

    loop {
        terminal.draw(|f| app.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if let Err(e) = app.handle_event(Event::Key(key)) {
                // If no more cards, exit
                if e.to_string() == "No more cards due for review" {
                    break;
                }
                return Err(e);
            }
        }
    }
    Ok(())
}