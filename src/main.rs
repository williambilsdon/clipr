use std::io;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders, Paragraph}, text::Text};

struct State {
    inputs: String
}


fn app<B: Backend>(terminal: &mut Terminal<B>, mut state: State) -> io::Result<()> {
    loop {
        terminal.draw(|f|  {
            let size = f.size();
            let block = Block::default()
                .title("Clipr")
                .borders(Borders::ALL);
            
            let text = Text::from(state.inputs.clone());
            let paragraph = Paragraph::new(text);
            let inner_size = block.inner(f.size());
            
            f.render_widget(block, size);
            f.render_widget(paragraph, inner_size);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char(c) = key.code {
                state.inputs.push(c);
            }
        }
    }

}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let state = State { inputs: String::new() };

    let _ = app(&mut terminal, state);

    Ok(())
}