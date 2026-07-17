use eframe::{NativeOptions, egui};

mod core;
use core::window::{AppState, PineDeck};

mod panels;

fn main() -> eframe::Result {
    let native_options: NativeOptions = NativeOptions::default();

    eframe::run_native(
        "Pinedeck",
        native_options,
        Box::new(|_cc| Ok(Box::new(PineDeck::default()))),
    )
}

impl eframe::App for PineDeck {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match &self.app_state {
            AppState::HomeScreen => {
                panels::home::render(ui, self);
            }
            AppState::DashboardScreen => {
                panels::dashboard::render(ui, self);
            }
        }
    }
}
