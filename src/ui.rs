mod create;
mod list;

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
};

use crate::model::app::{App, Mode};

trait Draw {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>>;
}

trait Input {
    fn input(&mut self, event: KeyEvent) -> Result<(), Box<dyn Error>>;
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    setup_terminal()?;

    let mut state = App::new();
    let mut list_mode = list::List::new();

    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Please select a mode: Create (c), List (l)")
                .block(Block::default().title("Menu").borders(Borders::ALL));
            frame.render_widget(greeting, frame.size());
        })?;

        match state.mode {
            Mode::Menu => {
                //TODO:
                {}
            }
            Mode::Create => create::create_mode(terminal)?,
            Mode::List => list_mode.draw(terminal)?,
        }

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('c') => {
                    state.mode = Mode::Create;
                }
                KeyCode::Char('l') => {
                    state.mode = Mode::List;
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    restore_terminal(terminal)?;

    Ok(())
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(terminal.show_cursor()?)
}
