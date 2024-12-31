// ui.rs
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui::prelude::*;
use crate::model::{Card};

pub fn draw_frame(f: &mut Frame) -> Rect {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
            Constraint::Length(1),    // Title
            Constraint::Min(0),       // Content
            Constraint::Length(1),    // Status
            ])
    .split(f.area());

    // Title
    f.render_widget(
        Block::default()
        .borders(Borders::ALL)
        .title(" 안녕 "), 
        layout[0]
        );

    // Status
    f.render_widget(
        Block::default()
        .borders(Borders::ALL)
        .title(" Status ")
        .title_alignment(Alignment::Right),
        layout[2]
        );

    // Return the content layout for other views to use
    layout[1]
}

pub fn draw_hint(f: &mut Frame, card: &Card) {
    let content_area = draw_frame(f);

    let inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(30),
           Constraint::Max(1),      // Front text
           Constraint::Min(0),      
           ])
    .split(content_area);

    f.render_widget(
        Paragraph::new(card.front.as_str())
        .alignment(Alignment::Center),
        inner_layout[1]
        );
}

pub fn draw_full(f: &mut Frame, card: &Card) {
    let content_area = draw_frame(f);
    draw_hint(f, card);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Max(1),
            Constraint::Min(0),
            ])
        .split(content_area);

    let back_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            ])
        .split(inner_layout[2]);

    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),    
            Constraint::Percentage(33),    
            Constraint::Percentage(33),    
            ])
        .split(back_layout[3]);

    f.render_widget(
        Paragraph::new(&*card.back)
        .alignment(Alignment::Center),
        back_layout[1]
        );

    f.render_widget(
        Paragraph::new(card.explanation.as_deref().unwrap_or(""))
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Explanation").borders(Borders::ALL))
        .alignment(Alignment::Center),
        info_layout[0]
        );

    let examples = card.examples.as_ref()
    .map(|e| e.iter()
        .map(|ex| format!("{} - {}\n", ex.sentence, ex.translation))
        .collect::<String>())
    .unwrap_or_default();

    f.render_widget(
        Paragraph::new(examples)
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Examples").borders(Borders::ALL))
        .alignment(Alignment::Center),
        info_layout[1]
        );

    f.render_widget(
        Paragraph::new(card.notes.as_deref().unwrap_or(""))
        .wrap(Wrap { trim: true })
        .block(Block::new().title("Notes").borders(Borders::ALL))
        .alignment(Alignment::Center),
        info_layout[2]
        );
}
