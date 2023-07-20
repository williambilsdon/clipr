use std::{error::Error, path::Path, fs, io::Write};

use super::ROOT_ADDR;

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
