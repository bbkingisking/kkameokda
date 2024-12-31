use crate::model::Card;
use ratatui::prelude::*;
use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::draw_hint;
use crate::ui::draw_full;
use crate::utilities::current_unix_time;
use crate::ui::draw_frame;
use crate::model::Deck;

pub enum CardState {
    Hint,
    Full,
}

pub struct App {
    pub decks: Vec<Deck>,
    pub state: CardState,
    pub remembered_count: u32,
    pub forgotten_count: u32,
    pub reversed: bool,
    due_cards: Vec<(Card, String)>,
    current_card: Option<(Card, String)>,
}

impl App {
    pub fn new(decks: Vec<Deck>) -> Self {
        let mut app = Self {
            decks,
            state: CardState::Hint,
            remembered_count: 0,
            forgotten_count: 0,
            reversed: rand::random(),
            due_cards: Vec::new(),
            current_card: None,
        };
        app.refresh_due_cards();
        if !app.due_cards.is_empty() {
            app.current_card = Some(app.due_cards[0].clone());
        }
        app
    }

    fn refresh_due_cards(&mut self) {
        let current_time = current_unix_time();
        let mut cards = Vec::new();
        
        for deck in &self.decks {
            cards.extend(
                deck.cards.iter()
                    .filter(|card| card.next_review < Some(current_time))
                    .map(|card| (card.clone(), deck.name.clone()))
            );
            
            for subdeck in &deck.subdecks {
                cards.extend(
                    subdeck.cards.iter()
                        .filter(|card| card.next_review < Some(current_time))
                        .map(|card| (card.clone(), subdeck.name.clone()))
                );
            }
        }
        
        self.due_cards = cards;
    }

    pub fn due_cards_count(&self) -> usize {
        self.due_cards.len()
    }

    fn get_card_mut(&mut self, card_to_find: &Card) -> Option<(&mut Card, &str)> {
        let current_time = current_unix_time();
        
        for deck in &mut self.decks {
            for card in &mut deck.cards {
                if card.next_review < Some(current_time) && 
                   card.front == card_to_find.front && 
                   card.back == card_to_find.back {
                    return Some((card, &deck.name));
                }
            }
            
            for subdeck in &mut deck.subdecks {
                for card in &mut subdeck.cards {
                    if card.next_review < Some(current_time) && 
                       card.front == card_to_find.front && 
                       card.back == card_to_find.back {
                        return Some((card, &subdeck.name));
                    }
                }
            }
        }
        
        None
    }

    fn current_card(&self) -> Option<(&Card, &str)> {
        self.current_card.as_ref()
            .map(|(card, name)| (card, name.as_str()))
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
        
        // Clone the current card first to avoid the borrow conflict
        let current_card = match &self.current_card {
            Some((card, _)) => card.clone(),
            None => return Ok(()),
        };

        // Now we can mutably borrow self
        if let Some((card, _)) = self.get_card_mut(&current_card) {
            card.calculate_next_review(current_time, remembered)?;
        }

        self.refresh_due_cards();
        
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

    fn next_card(&mut self) {
        if !self.due_cards.is_empty() {
            let next_card = if let Some((current_card, _)) = &self.current_card {
                // Find the first card that's different from the current one
                self.due_cards.iter()
                    .find(|(card, _)| 
                        card.front != current_card.front || 
                        card.back != current_card.back
                    )
                    .or(self.due_cards.first())
                    .cloned()
            } else {
                self.due_cards.first().cloned()
            };
            
            if let Some(card) = next_card {
                self.current_card = Some(card);
                self.state = CardState::Hint;
                self.reversed = rand::random();
            }
        }
    }

    fn current_deck_name(&self) -> Option<&str> {
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