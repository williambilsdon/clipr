use std::{error::Error, path::Path, fs, io::Write};

use ratatui::widgets::ListState;

pub const ROOT_ADDR: &str = "/home/wbilsdon/Documents/clipr/";

pub struct File {
    pub content: String,
    pub name: String
}

impl File {
    pub fn new() -> Self {
        File {
            content: String::new(),
            name: String::new()
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let string_path = format!("{}{}", ROOT_ADDR, self.name.as_str());
        let path = Path::new(string_path.trim());

        let mut output = fs::File::create(path)?;
        output.write_all(self.content.as_bytes())?;

        Ok(())
    }
}

pub enum Mode {
    Menu,
    List,
    Create
}

pub struct State {
    pub mode: Mode,
    pub items: Vec<String>,
    pub selected: ListState
}

impl State {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        State {
            mode: Mode::Menu,
            items: Vec::new(),
            selected: list_state
        }
    }
}
