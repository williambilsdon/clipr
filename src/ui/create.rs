use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io::Stdout};

use crate::model::file::File;

use super::{Draw, Input};

struct Create {
    file: File,
    save_popup: bool,
}

impl Create {
    pub fn new() -> Self {
        Create {
            file: File::new(),
            save_popup: false,
        }
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    //TODO: Add save input here
    fn show_save_popup<B: Backend>(&mut self, frame: &mut Frame<B>) {
        if self.save_popup {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = self.centered_rect(60, 20, frame.size());
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(block, area);
        };
    }
}

impl Draw for Create {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let name_file_text = format!("Name the new file: {}", self.file.name);

        terminal.draw(|frame| {
            let name_file = Paragraph::new(name_file_text)
                .block(Block::default().title("Create").borders(Borders::ALL));
            frame.render_widget(name_file, frame.size());
            self.show_save_popup(frame);
        })?;

        Ok(())
    }
}

impl Input for Create {
    fn input(&mut self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
        match event {
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => self.save_popup = !self.save_popup,
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => {
                self.file.content.push(c);
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } if self.save_popup => {
                self.save_popup = !self.save_popup;
                self.file.write()?
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                self.file.content.push('\n');
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.file.content.pop();
            }
            KeyEvent {
                code: KeyCode::Esc, ..
            } => {
                self.file.write()?;
            }
            _ => {}
        }

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
