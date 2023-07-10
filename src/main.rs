use std::{io::{Write, Stdout, self}, path::Path, fs, error::Error};
use crossterm::{event::{Event, KeyCode, read, KeyEvent}, terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, execute, };
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

fn main() -> Result<(), Box<dyn Error>>{

    let mut terminal = setup_terminal()?;
    let mut state = State::new();
    run(&mut terminal, &mut state)?;
    
    match state.mode {
        Mode::Menu => todo!(),
        Mode::Create => create_mode(&mut terminal)?,
        Mode::Select => println!("Select is not implemented yet, program closing."),
    }

    restore_terminal(&mut terminal)?;

    Ok(())
}

fn create_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
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
        let content_str = format!("{}", &file.content);

        terminal.draw(|frame| {
            let contents = Paragraph::new(content_str);
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

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), Box<dyn Error>> {
    Ok(
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
        }
    )
}

