// utilities.rs
use std::time::{SystemTime, UNIX_EPOCH};
use crate::model::Deck;

pub fn current_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_secs()
}

pub fn print_deck_structure(deck: &Deck, indent: usize) {
    let indent_str = " ".repeat(indent);
    println!("{}└─ {} ({} cards)", indent_str, deck.name, deck.cards.len());
    
    if let Some(subdecks) = &deck.subdecks {
        for subdeck in subdecks {
            print_deck_structure(subdeck, indent + 2);
        }
    }
}