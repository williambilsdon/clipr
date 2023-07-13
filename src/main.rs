use std::{io::{Stdout, self}, error::Error};
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, execute};
use ratatui::{backend::CrosstermBackend, Terminal};

mod ui;

fn main() -> Result<(), Box<dyn Error>>{

    let mut terminal = setup_terminal()?;
    let mut state = ui::State::new();
    ui::run(&mut terminal, &mut state)?;
    
    //TODO: Move this logic out to ui::run
    match state.mode {
        ui::Mode::Menu => todo!(),
        ui::Mode::Create => ui::create_mode(&mut terminal)?,
        ui::Mode::Select => println!("Select is not implemented yet, program closing."),
    }

    restore_terminal(&mut terminal)?;

    Ok(())
}


fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

