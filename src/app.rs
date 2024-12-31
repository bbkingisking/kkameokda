// app.rs
use crate::model::Card;
use ratatui::prelude::*;
use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::draw_hint;
use crate::ui::draw_full;
use crate::utilities::current_unix_time;
use crate::ui::draw_frame;
use crate::model::Deck;
use crate::app_runner::collect_due_cards;

pub enum CardState {
    Hint,
    Full,
}

pub struct App {
    pub decks: Vec<Deck>,
    pub current_card_index: usize,
    pub state: CardState,
    pub remembered_count: u32,
    pub forgotten_count: u32,
    pub reversed: bool,
}

impl App {
    pub fn new(decks: Vec<Deck>) -> Self {
        Self {
            decks,
            current_card_index: 0,
            state: CardState::Hint,
            remembered_count: 0,
            forgotten_count: 0,
            reversed: rand::random(),
        }
    }

    pub fn due_cards_count(&self) -> usize {
        collect_due_cards(&self.decks, current_unix_time()).len()
    }

    fn get_card_mut(&mut self, index: usize) -> Option<(&mut Card, &str)> {
        let current_time = current_unix_time();
        let mut found_index = 0;
        
        for deck in &mut self.decks {
            for card in &mut deck.cards {
                if card.next_review < Some(current_time) {
                    if found_index == index {
                        return Some((card, &deck.name));
                    }
                    found_index += 1;
                }
            }
            
            // Remove the if let and iterate directly
            for subdeck in &mut deck.subdecks {
                for card in &mut subdeck.cards {
                    if card.next_review < Some(current_time) {
                        if found_index == index {
                            return Some((card, &subdeck.name));
                        }
                        found_index += 1;
                    }
                }
            }
        }
        
        None
    }

    fn current_card(&self) -> Option<(&Card, &str)> {
        collect_due_cards(&self.decks, current_unix_time())
            .get(self.current_card_index)
            .copied()
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char(' ') => self.toggle_state(),
                KeyCode::Enter => self.review_card(true)?,
                KeyCode::Char('f') => self.review_card(false)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn review_card(&mut self, remembered: bool) -> Result<()> {
        let current_time = current_unix_time();
        
        if remembered {
            self.remembered_count += 1;
        } else {
            self.forgotten_count += 1;
        }
        
        if let Some((card, _)) = self.get_card_mut(self.current_card_index) {
            card.calculate_next_review(current_time, remembered)?;
        }       
        let due_cards_count = self.due_cards_count();
        if due_cards_count == 0 {
            return Err(color_eyre::eyre::eyre!("No more cards due for review"));
        }
        
        self.next_card();
        Ok(())
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            CardState::Hint => CardState::Full,
            CardState::Full => CardState::Hint,
        };
    }

    pub fn next_card(&mut self) {
        let due_count = self.due_cards_count();
        if due_count > 0 {
            self.current_card_index = (self.current_card_index + 1) % due_count;
            self.state = CardState::Hint;
            self.reversed = rand::random(); // New random value for each card
        }
    }

    fn current_deck_name(&self) -> Option<&str> {
        // Change the return type to Option<String> if you want to keep the full path
        self.current_card().map(|(_, deck_path)| deck_path)
    }

    pub fn draw(&self, f: &mut Frame) {
        let total_due = self.due_cards_count();
        draw_frame(f, total_due, self.remembered_count, self.forgotten_count, self.current_deck_name());

        if let Some((card, _)) = self.current_card() {
            match self.state {
                CardState::Hint => draw_hint(f, card, self.reversed),
                CardState::Full => draw_full(f, card),
            }
        }
    }
}