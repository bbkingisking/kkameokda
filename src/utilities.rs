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
    
    for subdeck in &deck.subdecks {
        let subdeck_name = subdeck.name.split('/').last().unwrap_or(&subdeck.name);
        let indent_str = " ".repeat(indent + 2);
        println!("{}└─ {} ({} cards)", indent_str, subdeck_name, subdeck.cards.len());
    }
}