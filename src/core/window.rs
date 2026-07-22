pub struct PineDeck {
    pub app_state: AppState,
    pub is_connected_to_board: bool,
}

pub enum AppState {
    HomeScreen,
    DashboardScreen,
    TerminalScreen,
    CodeScreen,
    FilesScreen,
    ProcessesScreen,
}

impl Default for PineDeck {
    fn default() -> Self {
        Self {
            app_state: AppState::DashboardScreen,
            is_connected_to_board: true,
        }
    }
}
