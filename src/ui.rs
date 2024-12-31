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

pub fn draw_frame(f: &mut Frame) -> Rect {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" 까먹다 ")
        .title(
            Title::from(" Status ")
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        );
    
    // Get the outer area
    let area = f.area();
    
    // Render the main border
    let inner_area = main_block.inner(area);  // Get the inner area accounting for borders
    f.render_widget(main_block, area);
    
    // Create the layout inside the bordered area using inner_area
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),    
            Constraint::Min(0),       

        ])
        .split(inner_area);  // Use inner_area instead of f.area()
    
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
