use std::{io::Stdout, error::Error, fs};
use crossterm::event::{read, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};
use crate::ui::model::ROOT_ADDR;

pub fn list_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    loop {
        let files = fs::read_dir(ROOT_ADDR)?;
        let mut file_list = String::new();
        for file in files.into_iter() {
            let name = format!("{}\n", file.unwrap().file_name().to_str().unwrap());
            file_list.push_str(name.as_str());
            
        };
        
        terminal.draw(|frame| {
            let name_file = Paragraph::new(file_list.as_str());
            frame.render_widget(name_file, frame.size());
        })?;

        if let Event::Key(event) = read()? {
                if event.code == KeyCode::Esc { break }
            }
        }
    Ok(())
}