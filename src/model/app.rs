use ratatui::widgets::ListState;

pub enum Mode {
    Menu,
    List,
    Create,
}

pub struct App {
    pub mode: Mode,
    pub items: Vec<String>,
    pub selected: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            mode: Mode::Menu,
            items: Vec::new(),
            selected: list_state,
        }
    }
}
