pub struct PineDeck {
    pub app_state: AppState,
}

pub enum AppState {
    HomeScreen,
    DashboardScreen,
}

impl Default for PineDeck {
    fn default() -> Self {
        Self {
            app_state: AppState::HomeScreen,
        }
    }
}
