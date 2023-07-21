use std::{io::Stdout, error::Error, fs};
use crossterm::event::{read, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::{List, ListItem, Block, Borders}, text::{Line, Span}, style::{Style, Color, Modifier}};

use crate::model::{ROOT_ADDR, state::State};

pub fn list_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut State) -> Result<(), Box<dyn Error>> {
    loop {
        let dir = fs::read_dir(ROOT_ADDR)?;
        state.items = dir.filter_map(|file| 
            file.ok().and_then(|f| 
                f.file_name().to_str().map(String::from)
            )
        ).collect();
      
        let files: Vec<ListItem> = state.items.iter().map(|item| 
            ListItem::new(Line::from(Span::raw(item.to_string())))
        ).collect();
        
        terminal.draw(|frame| {
            let name_file = List::new(files).block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(name_file,frame.size(), &mut state.selected);
        })?;

        if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if let Some(selected) = state.selected.selected() {
                            if selected > 0 {
                                state.selected.select(Some(selected - 1));
                            } else {
                                state.selected.select(Some(state.items.len().saturating_sub(1)));
                            }
    
                        }
                    },
                    KeyCode::Down => {
                        if let Some(selected) = state.selected.selected() {
                            if selected >= state.items.len().saturating_sub(1) {
                                state.selected.select(Some(0));
                            } else {
                                state.selected.select(Some(selected + 1));
                            }

                        }
                    },
                    _ => {}
            }
        }
    }
    Ok(())
}