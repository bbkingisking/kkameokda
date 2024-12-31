// app_runner.rs
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    DefaultTerminal,
};
use crate::model::Deck;
use crate::app::App;
use color_eyre::Result;

pub fn run(mut terminal: DefaultTerminal, decks: Vec<Deck>) -> Result<App> {
    let mut app = App::new(decks);

    if app.due_cards_count() == 0 {
        return Err(color_eyre::eyre::eyre!("No cards due for review"));
    }

    loop {
        terminal.draw(|f| app.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(app);
            }
            if let Err(e) = app.handle_event(Event::Key(key)) {
                if e.to_string() == "No more cards due for review" {
                    return Ok(app);
                }
                return Err(e);
            }
        }
    }
}
