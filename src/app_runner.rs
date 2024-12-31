// app_runner.rs
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    DefaultTerminal,
};
use crate::model::{Card};
use crate::utilities::current_unix_time;
use crate::model::Deck;
use crate::app::App;
use color_eyre::Result;

fn collect_due_cards<'a>(deck: &'a mut Deck, current_time: u64) -> Vec<(&'a mut Card, &'a str)> {
    let mut cards = deck.cards.iter_mut()
        .filter(|card| card.next_review < Some(current_time))
        .map(|card| (card, deck.name.as_str()))
        .collect::<Vec<_>>();
    
    if let Some(subdecks) = &mut deck.subdecks {
        for subdeck in subdecks {
            cards.extend(collect_due_cards(subdeck, current_time));
        }
    }
    
    cards
}

pub fn run(mut terminal: DefaultTerminal, mut decks: Vec<Deck>) -> Result<()> {
    let current_time = current_unix_time();
    
    let due_cards: Vec<(&mut Card, &str)> = decks.iter_mut()
        .flat_map(|deck| collect_due_cards(deck, current_time))
        .collect();
    
    if due_cards.is_empty() {
        return Err(color_eyre::eyre::eyre!("No cards due for review"));
    }
    
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
