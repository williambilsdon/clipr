use crossterm::{event::{Event, KeyCode, read}, terminal};

struct EndProg;

impl Drop for EndProg {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("error dropping raw mode");
    }
}
fn main() {
    let _prog_manager = EndProg;

    terminal::enable_raw_mode().expect("error enabling raw mode");

    loop {
        match read().expect("error reading input") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(input) => println!("input is: {}", input),
                    _ => todo!()
                }
            },
            _ => todo!(),
        }
    }
}

