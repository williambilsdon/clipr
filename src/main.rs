use std::{io::{stdout, Write}, path::Path, fs};
use crossterm::{event::{Event, KeyCode, read, KeyEvent}, terminal::{self, enable_raw_mode, disable_raw_mode}, execute, cursor::MoveTo, Result};

const ROOT_ADDR: &str = "/home/wbilsdon/Documents/clipr/";

struct File {
    content: String,
    name: String
}

impl File {
    pub fn new() -> File {
        File {
            content: String::new(),
            name: String::new()
        }
    }
}

fn main() -> Result<()>{
    
    let mut file = File::new();

    println!("Name the new file :");
    let _ = std::io::stdin().read_line(&mut file.name).unwrap();

    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), crossterm::cursor::Hide)?;
    enable_raw_mode()?;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveTo(0, 0), crossterm::style::Print(&file.content))?;
        execute!(stdout(), MoveTo(file.content.len() as u16, 0))?;
        stdout().flush()?;
       
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
                    let string_path = format!("{}{}", ROOT_ADDR, file.name.as_str());
                    let path = Path::new(string_path.trim());

                    let mut output = fs::File::create(path)?;
                    output.write_all(file.content.as_bytes())?;

                    break
                },
                _ => {}
            }
        } 
    }

    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?;
    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

