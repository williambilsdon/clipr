use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io::stdout};

mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = ui::App::new(&mut terminal);
    app.run()?;
    Ok(())
}
