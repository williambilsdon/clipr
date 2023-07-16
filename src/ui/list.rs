use std::{io::Stdout, error::Error, fs};
use crossterm::event::{read, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::{List, ListItem}, text::{Line, Span}};
use crate::ui::model::ROOT_ADDR;

pub fn list_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    loop {
        let files = fs::read_dir(ROOT_ADDR)?;
        let file_list: Vec<ListItem> = files.map(|res| res.map(|f| {
            ListItem::new(Line::from(Span::raw(format!("{}", f.file_name().to_str().unwrap()))))
        }).unwrap()).collect();
        
        terminal.draw(|frame| {
            let name_file = List::new(file_list);
            frame.render_widget(name_file, frame.size());
        })?;

        if let Event::Key(event) = read()? {
                if event.code == KeyCode::Esc { break }
            }
        }
    Ok(())
}