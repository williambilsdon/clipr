mod create;
mod list;

use std::{io::{Stdout, self}, error::Error};
use crossterm::{event::{Event, read, KeyCode, EnableMouseCapture, DisableMouseCapture}, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Paragraph, Block, Borders}};

use crate::model::state::{State, Mode};

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    setup_terminal()?;

    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Please select a mode: Create (c), List (l)").block(Block::default().title("Menu").borders(Borders::ALL));
            frame.render_widget(greeting, frame.size());
        })?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('c') => {
                    state.mode = Mode::Create; 
                },
                KeyCode::Char('l') => {
                    state.mode = Mode::List;
                }
                KeyCode::Esc => break,
                _ => {},
            }
        }

        match state.mode {
            Mode::Menu => {
                //TODO:
                {}
            },
            Mode::Create => create::create_mode(terminal)?,
            Mode::List => list::list_mode(terminal, &mut state)?,
        }
    
    };

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
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(terminal.show_cursor()?)
}



