mod create;
mod list;
mod model;

use std::{io::Stdout, error::Error};
use crossterm::event::{Event, read, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::Paragraph};
use model::{Mode, State};

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Please select a mode: Create (c), List (l)");
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
            Mode::Menu => todo!(),
            Mode::Create => create::create_mode(terminal)?,
            Mode::List => list::list_mode(terminal)?,
        }
    
    };

    Ok(())
}



