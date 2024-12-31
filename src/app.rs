// app.rs
use crate::model::Card;
use ratatui::prelude::*;
use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::draw_hint;
use crate::ui::draw_full;

pub enum CardState {
    Hint,
    Full,
}

pub struct App<'a> {
    pub cards: Vec<(&'a Card, &'a str)>, // (card, deck_name)
    pub current_card_index: usize,
    pub state: CardState,
}

impl<'a> App<'a> {
    pub fn new(cards: Vec<(&'a Card, &'a str)>) -> Self {
        Self {
            cards,
            current_card_index: 0,
            state: CardState::Hint,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char(' ') => self.toggle_state(),
                KeyCode::Right => self.next_card(),
                KeyCode::Left => self.prev_card(),
                _ => {}
            }
        }
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
        match self.state {
            CardState::Hint => draw_hint(f, self.current_card()),
            CardState::Full => draw_full(f, self.current_card()),
        }
    }
}
