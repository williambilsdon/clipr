use std::{io::stdout, error::Error};
use ratatui::{backend::CrosstermBackend, Terminal};

mod ui;
mod model;

fn main() -> Result<(), Box<dyn Error>>{

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    ui::run(&mut terminal)?;
    Ok(())
}




