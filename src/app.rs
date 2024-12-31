// app.rs
use crate::model::Card;
use ratatui::prelude::*;
use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::draw_hint;
use crate::ui::draw_full;
use crate::utilities::current_unix_time;
use crate::ui::draw_frame;

pub enum CardState {
    Hint,
    Full,
}

pub struct App<'a> {
    pub cards: Vec<(&'a mut Card, &'a str)>, // Change to mut Card
    pub current_card_index: usize,
    pub state: CardState,
}

impl<'a> App<'a> {
    pub fn new(cards: Vec<(&'a mut Card, &'a str)>) -> Self {  // Update constructor
        Self {
            cards,
            current_card_index: 0,
            state: CardState::Hint,
        }
    }

    pub fn due_cards_count(&self) -> usize {
        self.cards.len()
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char(' ') => self.toggle_state(),
                KeyCode::Right => self.next_card(),
                KeyCode::Left => self.prev_card(),
                KeyCode::Enter => self.review_card(true)?,  // Remember
                KeyCode::Char('f') => self.review_card(false)?,  // Forget
                _ => {}
            }
        }
        Ok(())
    }

    fn review_card(&mut self, remembered: bool) -> Result<()> {
        let current_time = current_unix_time();
        
        // Get mutable reference to current card
        // Note: We need to index into cards vector directly since we're mutating
        let (card, _) = &mut self.cards[self.current_card_index];
        
        // Calculate next review time and save
        card.calculate_next_review(current_time, remembered)?;
        
        // Move to next card
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
        self.current_card_index = (self.current_card_index + 1) % self.cards.len();
        self.state = CardState::Hint;
    }

    pub fn prev_card(&mut self) {
        self.current_card_index = if self.current_card_index == 0 {
            self.cards.len() - 1
        } else {
            self.current_card_index - 1
        };
        self.state = CardState::Hint;
    }

    pub fn current_card(&self) -> &Card {
        self.cards[self.current_card_index].0
    }

    pub fn current_deck_name(&self) -> &str {
        self.cards[self.current_card_index].1
    }

    pub fn draw(&self, f: &mut Frame) {
        let total_due = self.due_cards_count();
        draw_frame(f, total_due);

        match self.state {
            CardState::Hint => draw_hint(f, self.current_card()),
            CardState::Full => draw_full(f, self.current_card()),
        }
    }
}
