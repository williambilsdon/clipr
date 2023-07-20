use ratatui::widgets::ListState;

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