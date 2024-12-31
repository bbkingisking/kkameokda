use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    DefaultTerminal,
};
use crate::model::{Card};
use crate::utilities::current_unix_time;
use rand::seq::SliceRandom;
use crate::model::Deck;
use crate::app::App;
use color_eyre::Result;

pub fn run(mut terminal: DefaultTerminal, decks: Vec<Deck>) -> Result<()> {
    let current_time = current_unix_time();
    
    // Helper function to get cards from a deck and its subdecks recursively
    fn collect_due_cards<'a>(deck: &'a Deck, current_time: u64) -> Vec<(&'a Card, &'a str)> {
        let mut cards = deck.cards.iter()
        .filter(|card| card.next_review < Some(current_time))
        .map(|card| (card, deck.name.as_str()))
        .collect::<Vec<_>>();
        
        if let Some(subdecks) = &deck.subdecks {
            for subdeck in subdecks {
                cards.extend(collect_due_cards(subdeck, current_time));
            }
        }
        
        cards
    }
    
    // Get due cards from all decks and their subdecks
    let due_cards: Vec<(&Card, &str)> = decks.iter()
    .flat_map(|deck| collect_due_cards(deck, current_time))
    .collect();
    
    if due_cards.is_empty() {
        return Err(color_eyre::eyre::eyre!("No cards due for review"));
    }
    
    let (current_card, deck_name) = due_cards.choose(&mut rand::thread_rng())
    .expect("No cards due for review");

    
    let mut app = App::new(due_cards);

    loop {
        terminal.draw(|f| app.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            app.handle_event(Event::Key(key))?;
        }
    }
    Ok(())
}
