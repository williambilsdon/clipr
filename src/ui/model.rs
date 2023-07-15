use std::{error::Error, path::Path, fs, io::Write};

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
