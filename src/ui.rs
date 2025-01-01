// ui.rs
use ratatui::{
    layout::{Alignment, Constraint, Layout, Flex, Rect},
    widgets::{Borders, BorderType, Paragraph, Wrap, Clear},
    Frame,
};
use ratatui::prelude::*;
use crate::model::{Card};
use ratatui::widgets::{
    block::{Position, Title},
    Block,
};
use ratatui::style::{Color, Style};

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn draw_frame(f: &mut Frame, remaining: usize, remembered: u32, forgotten: u32, current_deck: Option<&str>, show_shortcuts: bool) {
    let mut main_block = Block::default()
        .borders(Borders::ALL)
        .title(
            Title::from(
                Line::from(vec![
                    Span::raw(" 까먹다 "),
                    Span::raw("│ "),
                    Span::styled(
                        format!(" {} ", current_deck.unwrap_or("No Deck")),
                        Style::default().fg(Color::Yellow)
                    ),
                    Span::raw(" "),                
                ])
            )
        );

    // Simplified shortcuts string with just the ? key
    let shortcuts = Line::from(vec![
        Span::raw(" "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(": Show shortcuts "),
    ]);

    main_block = main_block.title(
        Title::from(shortcuts)
            .alignment(Alignment::Left)
            .position(Position::Bottom)
    );

    main_block = main_block.title(
        Title::from(Line::from(vec![
            Span::raw(" ("),
            Span::styled(
                format!("{}", remembered),
                Style::default().fg(Color::Green)
            ),
            Span::raw("/"),
            Span::styled(
                format!("{}", forgotten),
                Style::default().fg(Color::Red)
            ),
            Span::raw(") | "),
            Span::styled(
                format!("{} ", remaining),
                Style::default().fg(Color::Green)
            ),
        ]))
        .alignment(Alignment::Right)
        .position(Position::Bottom)
    );
    
    f.render_widget(main_block, f.area());

    // Render shortcuts popup if enabled
    if show_shortcuts {
        let area = popup_area(f.area(), 60, 40);
        f.render_widget(Clear, area);
        
    let shortcuts_text = vec![
        Line::from("\n"), // Empty line at top for spacing
        Line::from(vec![
            Span::styled("Space", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle card view"),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Mark as remembered"),
        ]),
        Line::from(vec![
            Span::styled("f", Style::default().fg(Color::Yellow)),
            Span::raw(": Mark as forgotten"),
        ]),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
        Line::from(vec![
            Span::styled("Esc/?", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle shortcuts"),
        ]),
    ];
            
        let popup = Paragraph::new(shortcuts_text)
            .block(Block::default()
                .title("Shortcuts")
                .borders(Borders::ALL).border_type(BorderType::Double))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(popup, area);
    }
}

pub fn draw_hint(f: &mut Frame, card: &Card, reversed: bool) {
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

    if reversed {
        f.render_widget(
            Paragraph::new(card.back.as_str())
                .alignment(Alignment::Center),
            inner_layout[1]
        );
    } else {
        f.render_widget(
            Paragraph::new(card.front.as_str())
                .alignment(Alignment::Center),
            inner_layout[1]
        );
    }
}

pub fn draw_full(f: &mut Frame, card: &Card, reversed: bool) {
    let area = f.area();

    // Get inner area accounting for the main frame's borders
    let inner_area = Block::default()
        .borders(Borders::ALL)
        .inner(area);

    // Get layout for front
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Max(1),      // Front text
            Constraint::Min(0),      
        ])
        .split(inner_area);

    // Draw front
    if reversed {
        f.render_widget(
            Paragraph::new(card.back.as_str())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            inner_layout[1]
        );
    } else {
        f.render_widget(
            Paragraph::new(card.front.as_str())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            inner_layout[1]
        );
    }

    // Get layout for back
    let back_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(inner_layout[2]);

    // Draw back
    if reversed {
        f.render_widget(
            Paragraph::new(card.front.as_str())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            back_layout[1]
        );
    } else {
        f.render_widget(
            Paragraph::new(card.back.as_str())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            back_layout[1]
        );
    }
    // Get layout for info
    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(create_dynamic_constraints(
            card.explanation.is_some(),
            !card.examples.as_ref().map_or(true, |e| e.is_empty()),
            card.notes.is_some()
        ))
        .split(back_layout[3]);

    let mut rendered_sections = 0;

    if let Some(explanation) = &card.explanation {
        f.render_widget(
            Paragraph::new(explanation.as_str())
                .wrap(Wrap { trim: true })
                .block(Block::new().title("Explanation").borders(Borders::ALL))
                .alignment(Alignment::Center),
            info_layout[rendered_sections]
        );
        rendered_sections += 1;
    }

    if let Some(examples) = &card.examples {
        if !examples.is_empty() {
            let examples_text = examples.iter()
                .map(|ex| format!("{} - {}\n", ex.sentence, ex.translation))
                .collect::<String>();
            
            f.render_widget(
                Paragraph::new(examples_text.as_str())
                    .wrap(Wrap { trim: true })
                    .block(Block::new().title("Examples").borders(Borders::ALL))
                    .alignment(Alignment::Center),
                info_layout[rendered_sections]
            );
            rendered_sections += 1;
        }
    }

    if let Some(notes) = &card.notes {
        f.render_widget(
            Paragraph::new(notes.as_str())
                .wrap(Wrap { trim: true })
                .block(Block::new().title("Notes").borders(Borders::ALL))
                .alignment(Alignment::Center),
            info_layout[rendered_sections]
        );
    }
}

fn create_dynamic_constraints(has_explanation: bool, has_examples: bool, has_notes: bool) -> Vec<Constraint> {
    let present_sections = [has_explanation, has_examples, has_notes];
    let count = present_sections.iter().filter(|&&x| x).count();
    
    match count {
        0 => vec![],
        1 => vec![Constraint::Percentage(100)],
        2 => vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        3 => vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ],
        _ => unreachable!(),
    }
}
