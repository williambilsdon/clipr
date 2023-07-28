use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io::Stdout};

use super::{file::File, App, Draw, Input, Mode};

pub struct Create {
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

    fn content_input(&mut self, event: KeyEvent, app: &mut App) -> Result<(), Box<dyn Error>> {
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
                self.file.write()?;
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
            } => app.mode = Mode::Menu,
            _ => {}
        }

        Ok(())
    }

    fn save_input(&mut self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
        match event {
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => {
                self.file.name.push(c);
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                self.save_popup = !self.save_popup;
                self.file.write()?;
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.file.content.pop();
            }
            KeyEvent {
                code: KeyCode::Esc, ..
            } => self.save_popup = !self.save_popup,
            _ => {}
        }

        Ok(())
    }

    fn show_save_popup<B: Backend>(&mut self, frame: &mut Frame<B>) {
        if self.save_popup {
            let block = Paragraph::new(self.file.name.to_string()).block(
                Block::default()
                    .title("Enter File Name")
                    .borders(Borders::ALL),
            );

            let area = centered_rect(60, 20, frame.size());
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
        terminal.draw(|frame| {
            let contents = Paragraph::new(self.file.content.to_string())
                .block(Block::default().title("Create").borders(Borders::ALL));
            frame.render_widget(contents, frame.size());
            self.show_save_popup(frame);
        })?;

        Ok(())
    }
}

impl Input for Create {
    fn input(&mut self, event: KeyEvent, app: &mut App) -> Result<(), Box<dyn Error>> {
        if self.save_popup {
            self.save_input(event)?
        } else {
            self.content_input(event, app)?
        }

        Ok(())
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
