use std::{io::{stdout, Write}, path::Path, fs};
use crossterm::{event::{Event, KeyCode, read, KeyEvent}, terminal::{self, enable_raw_mode, disable_raw_mode}, execute, cursor::MoveTo, Result, style::Print};

const ROOT_ADDR: &str = "/home/wbilsdon/Documents/clipr/";

struct File {
    content: String,
    name: String
}

impl File {
    pub fn new() -> Self {
        File {
            content: String::new(),
            name: String::new()
        }
    }
}

struct State {
    mode: Mode,
    // buffer: String
}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::Menu,
            // buffer: String::new()
        }
    }
}

enum Mode {
    Menu,
    Select,
    Create
}

fn main() -> Result<()>{
    
    let mut state = State::new();

    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), crossterm::cursor::Hide)?;
    enable_raw_mode()?;

    execute!(stdout(), Print("Please select a mode: Create (c), Select (s)"))?;

    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('c') => {
                    state.mode = Mode::Create; 
                    break
                },
                KeyCode::Char('s') => {
                    state.mode = Mode::Select;
                    break
                }
                KeyCode::Esc => break,
                _ => {},
            }
        }
    }

    match state.mode {
        Mode::Menu => todo!(),
        Mode::Create => create_mode()?,
        Mode::Select => println!("Select is not implemented yet, program closing."),
    }

    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?;
    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn create_mode() -> Result<()> {
    let mut file = File::new();
    let output_line = "Name the new file: ";

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveTo(0, 0), Print(output_line), crossterm::style::Print(&file.name))?;
        execute!(stdout(), MoveTo(file.name.len() as u16 + output_line.len() as u16, 0))?;
        stdout().flush()?;

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

    Ok(())
}

