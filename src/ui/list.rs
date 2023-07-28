use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List as ListTui, ListItem, ListState},
    Terminal,
};
use std::{error::Error, fs, io::Stdout};

use super::{App, Draw, Input, Mode, ROOT_ADDR};

pub struct List {
    items: Vec<String>,
    pub state: ListState,
}

impl List {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        List {
            items: Vec::new(),
            state: list_state,
        }
    }

    fn get_items(&mut self) -> Result<&Vec<String>, Box<dyn Error>> {
        let dir = fs::read_dir(ROOT_ADDR)?;
        self.items = dir
            .filter_map(|file| {
                file.ok()
                    .and_then(|f| f.file_name().to_str().map(String::from))
            })
            .collect();

        Ok(&self.items)
    }
}

impl Draw for List {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let files: Vec<ListItem> = self
            .get_items()?
            .iter()
            .map(|item| ListItem::new(Line::from(Span::raw(item.to_string()))))
            .collect();

        terminal.draw(|frame| {
            let name_file = ListTui::new(files)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(name_file, frame.size(), &mut self.state);
        })?;

        Ok(())
    }
}

impl Input for List {
    fn input(&mut self, event: KeyEvent, app: &mut App) -> Result<(), Box<dyn Error>> {
        match event.code {
            KeyCode::Esc => app.mode = Mode::Menu,
            KeyCode::Up => {
                if let Some(selected) = self.state.selected() {
                    if selected > 0 {
                        self.state.select(Some(selected - 1));
                    } else {
                        self.state.select(Some(self.items.len().saturating_sub(1)));
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.state.selected() {
                    if selected >= self.items.len().saturating_sub(1) {
                        self.state.select(Some(0));
                    } else {
                        self.state.select(Some(selected + 1));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}
