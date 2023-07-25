use crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io::Stdout};

use crate::model::file::File;

use super::Draw;

struct Create {
    file: File,
}

impl Create {
    pub fn new() -> Self {
        Create { file: File::new() }
    }
}

impl Draw for Create {
    fn draw(
        self: &mut Self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let name_file_text = format!("Name the new file: {}", self.file.name);

        terminal.draw(|frame| {
            let name_file = Paragraph::new(name_file_text)
                .block(Block::default().title("Create").borders(Borders::ALL));
            frame.render_widget(name_file, frame.size());
        })?;

        Ok(())
    }
}

pub fn create_mode(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::new();

    loop {
        let name_file_text = format!("Name the new file: {}", &file.name);

        terminal.draw(|frame| {
            let name_file = Paragraph::new(name_file_text)
                .block(Block::default().title("Create").borders(Borders::ALL));
            frame.render_widget(name_file, frame.size());
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char(c) => file.name.push(c),
                KeyCode::Backspace => {
                    file.name.pop();
                }
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    loop {
        terminal.draw(|frame| {
            let contents = Paragraph::new(file.content.to_string())
                .block(Block::default().title("Create").borders(Borders::ALL));
            frame.render_widget(contents, frame.size());
        })?;

        if let Event::Key(event) = read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                } => {
                    file.content.push(c);
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    file.content.push('\n');
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    file.content.pop();
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    file.write()?;
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
