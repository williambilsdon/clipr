use std::io::{stdout, Write};

use crossterm::{event::{Event, KeyCode, read}, terminal, execute, cursor::MoveTo, Result};

fn main() -> Result<()>{

    execute!(stdout(), terminal::EnterAlternateScreen)?;

    let mut buffer = String::new();

    execute!(stdout(), crossterm::cursor::Hide)?;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveTo(0, 0), crossterm::style::Print(&buffer))?;
        execute!(stdout(), MoveTo(buffer.len() as u16, 0))?;

        stdout().flush()?;
        
        match read().expect("error reading input") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(c) => {
                        buffer.push(c);
                    },
                    KeyCode::Backspace => {
                        buffer.pop();
                    },
                    KeyCode::Esc => break,
                    _ => {}
                }
            },
            _ => {},
        }

    }

    execute!(stdout(), crossterm::cursor::Show)?;
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

