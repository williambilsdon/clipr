use std::{fs, io::{Stdout, Write}, error::Error, path::Path};
use crossterm::event::{Event, read, KeyCode, KeyEvent};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::Paragraph};

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

pub enum Mode {
    Menu,
    Select,
    Create
}

pub struct State {
    pub mode: Mode,
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

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Please select a mode: Create (c), Select (s)");
            frame.render_widget(greeting, frame.size());
        })?;
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
    };
    Ok(
        ()
    )
}

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

