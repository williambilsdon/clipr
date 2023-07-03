use std::io::{stdout, Write};
use arboard::Clipboard;
use crossterm::{event::{Event, KeyCode, read, KeyEvent, KeyModifiers}, terminal::{self, enable_raw_mode, disable_raw_mode}, execute, cursor::MoveTo, Result};

fn main() -> Result<()>{

    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), crossterm::cursor::Hide)?;
    enable_raw_mode()?;

    let mut buffer = String::new();
    let mut clipboard = Clipboard::new().expect("error setting up clipboard");

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveTo(0, 0), crossterm::style::Print(&buffer))?;
        execute!(stdout(), MoveTo(buffer.len() as u16, 0))?;
        stdout().flush()?;
       
        if let Event::Key(event) = read()? {
            match event {
                KeyEvent{code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL, ..} => {
                    buffer.push_str(clipboard.get_text()
                                .expect("Error getting clipboard text")
                                .as_str()
                            )
                },
                KeyEvent{code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL, ..} => {
                    buffer.push_str("Saving file...")
                },
                KeyEvent{code: KeyCode::Char(c), ..} => {
                    buffer.push(c);
                },
                KeyEvent{code: KeyCode::Backspace, ..} => {
                    buffer.pop();
                },
                KeyEvent{code: KeyCode::Esc, .. } => break,
                _ => {}
            }
        } 
    }

    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?;
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    
    Ok(())
}

