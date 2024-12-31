// ui.rs
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Borders, Paragraph, Wrap},
    Frame,
};
use ratatui::prelude::*;
use crate::model::{Card};
use ratatui::widgets::{
    block::{Position, Title},
    Block,
};
use ratatui::style::{Color, Style};

pub fn draw_frame(f: &mut Frame, remaining: usize) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" 까먹다 ")
        .title(
            Title::from(Line::from(format!(" {} ", remaining)).style(Style::default().fg(Color::Green)))
                .alignment(Alignment::Right)
                .position(Position::Bottom)
        );
    
    f.render_widget(main_block, f.area());
}

pub fn draw_hint(f: &mut Frame, card: &Card) {
    let area = f.area();
    let inner_area = Block::default()
        .borders(Borders::ALL)
        .inner(area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Max(1),      
            Constraint::Min(0),      
        ])
        .split(inner_area);

    f.render_widget(
        Paragraph::new(card.front.as_str())
            .alignment(Alignment::Center),
        inner_layout[1]
    );
}

pub fn draw_full(f: &mut Frame, card: &Card) {
    let area = f.area();
    // Get inner area accounting for the main frame's borders
    let inner_area = Block::default()
        .borders(Borders::ALL)
        .inner(area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Max(1),      // Front text
            Constraint::Min(0),      
        ])
        .split(inner_area);  // Use inner_area instead of area

    // Draw front
    f.render_widget(
        Paragraph::new(card.front.as_str())
            .alignment(Alignment::Center),
        inner_layout[1]
    );

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
