use std::{io::Stdout, error::Error};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};

use crate::model::file::File;

pub fn create_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut file = File::new();

    loop {
        let name_file_text = format!("Name the new file: {}", &file.name);

        terminal.draw(|frame| {
            let name_file = Paragraph::new(name_file_text);
            frame.render_widget(name_file, frame.size());
        })?;
    
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char(c) => file.name.push(c),
                KeyCode::Backspace => {
                    file.name.pop();
                },
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    loop {
        terminal.draw(|frame| {
            let contents = Paragraph::new(file.content.to_string());
            frame.render_widget(contents, frame.size());
        })?;
    
        if let Event::Key(event) = read()? {
            match event {
                KeyEvent{code: KeyCode::Char(c), ..} => {
                    file.content.push(c);
                },
                KeyEvent{code: KeyCode::Enter, ..} => {
                    file.content.push('\n');
                },
                KeyEvent{code: KeyCode::Backspace, ..} => {
                    file.content.pop();
                },
                KeyEvent{code: KeyCode::Esc, .. } => {
                    file.write()?;
                    break
                },
                _ => {}
            }
        } 
    }

    Ok(())
}
