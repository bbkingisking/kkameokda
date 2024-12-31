// ui.rs
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use ratatui::prelude::*;
use crate::model::{Card};
use crate::utilities::current_unix_time;
use rand::seq::SliceRandom;
use crate::model::Deck;

pub fn run(mut terminal: DefaultTerminal, decks: Vec<Deck>) -> Result<()> {
    let current_time = current_unix_time();
    
    // Get due cards from all decks
    let due_cards: Vec<(&Card, &str)> = decks.iter()
        .flat_map(|deck| {
            deck.cards.iter()
                .filter(|card| card.next_review < Some(current_time))
                .map(|card| (card, deck.name.as_str()))
        })
        .collect();
    
    if due_cards.is_empty() {
        return Err(color_eyre::eyre::eyre!("No cards due for review"));
    }
    
    let (current_card, _) = due_cards.choose(&mut rand::thread_rng())
        .expect("No cards due for review");

    loop {
        terminal.draw(|f| draw(f, current_card))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    // Add other key handling here
                    _ => {}
                }
            }
        }
    }
}

fn draw(frame: &mut Frame, card: &Card) {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
            Constraint::Length(1), // Title area
            Constraint::Min(0), // Main area
            Constraint::Length(1), // Status area
            ])
    .split(frame.area());

    let inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(30),
            Constraint::Max(1), // 1 line in the center of the screen
            Constraint::Min(0),
            ])
    .split(layout[1]);

    let back_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
            Constraint::Percentage(10), // Spacing
            Constraint::Percentage(10), // This is where the back goes
            Constraint::Percentage(40), // Spacing
            Constraint::Percentage(40),
            ])
    .split(inner_layout[2]);

    let misc_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        ])
    .split(back_layout[3]);

    // Render app's title
    frame.render_widget(Block::default()
        .borders(Borders::ALL)
        .title(" 안녕 "), layout[0]);

    // Render the front of the card
    frame.render_widget(
        Paragraph::new(&*card.front)
        .block(Block::new())
        .alignment(Alignment::Center),
        inner_layout[1]);

    // Render the back of the card
    frame.render_widget(
        Paragraph::new(&*card.back)
        .block(Block::new())
        .alignment(Alignment::Center),
        back_layout[1]);

    // Render the explanation
    frame.render_widget(
        Paragraph::new(card.explanation.as_deref().unwrap_or(""))
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Explanation").borders(Borders::ALL))
        .alignment(Alignment::Center),
        misc_layout[0]);

    let examples = card.examples.as_ref()
    .map(|e| e.iter()
        .map(|ex| format!("{} - {}\n", ex.sentence, ex.translation))
        .collect::<String>())
    .unwrap_or_default();

    // Render the examples
    frame.render_widget(
        Paragraph::new(examples)
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Examples").borders(Borders::ALL))
        .alignment(Alignment::Center),
        misc_layout[1]);

    // Render the notes
    frame.render_widget(
        Paragraph::new(card.notes.as_deref().unwrap_or(""))
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Notes").borders(Borders::ALL))
        .alignment(Alignment::Center),
        misc_layout[2]);

    // Render the status bar
    frame.render_widget(Block::default()
        .borders(Borders::TOP)
        .title(" Status ")
        .title_alignment(Alignment::Right),
        layout[2]);
}