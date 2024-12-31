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

pub fn print_session_summary(remembered: u32, forgotten: u32) {
    let total = remembered + forgotten;
    if total > 0 {
        println!("\nSession Summary");
        println!("---------------");
        println!("In this session, you remembered \x1b[32m{}\x1b[0m out of {} cards ({:.1}% retention rate)", 
            remembered, total,
            (remembered as f64 / total as f64 * 100.0)
        );
    }
}