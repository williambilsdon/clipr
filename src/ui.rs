mod create;
mod file;
mod list;

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
};

pub const ROOT_ADDR: &str = "/home/wbilsdon/Documents/clipr/";

enum Mode {
    Menu,
    List,
    Create,
    Quit,
}

pub struct App<'a> {
    mode: Mode,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        App {
            mode: Mode::Menu,
            terminal: terminal,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.setup_terminal()?;

        let mut list_mode = list::List::new();
        let mut create_mode = create::Create::new();

        loop {
            match self.mode {
                Mode::Menu => {
                    self.terminal.draw(|frame| {
                        let greeting = Paragraph::new("Please select a mode: Create (c), List (l)")
                            .block(Block::default().title("Menu").borders(Borders::ALL));
                        frame.render_widget(greeting, frame.size());
                    })?;
                }
                Mode::Create => create_mode.draw(self.terminal)?,
                Mode::List => list_mode.draw(self.terminal)?,
                Mode::Quit => break,
            }

            if let Event::Key(event) = read()? {
                match self.mode {
                    Mode::Menu => self.input(event)?,
                    Mode::Create => create_mode.input(event, &mut self.mode)?,
                    Mode::List => list_mode.input(event, &mut self.mode)?,
                    _ => {}
                }
            }
        }

        self.restore_terminal()?;

        Ok(())
    }

    fn setup_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    fn restore_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(self.terminal.show_cursor()?)
    }

    fn input(&mut self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
        match event.code {
            KeyCode::Char('c') => {
                self.mode = Mode::Create;
            }
            KeyCode::Char('l') => {
                self.mode = Mode::List;
            }
            KeyCode::Esc => self.mode = Mode::Quit,
            _ => {}
        }

        Ok(())
    }
}

trait Draw {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>>;
}

trait Input {
    fn input(&mut self, event: KeyEvent, mode: &mut Mode) -> Result<(), Box<dyn Error>>;
}
