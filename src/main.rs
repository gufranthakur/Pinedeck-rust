use eframe::{NativeOptions, egui};
use egui::accesskit::Role::Switch;

mod core;
use core::window::{AppState, PineDeck};

use crate::core::window::AppState::{DashboardScreen, FilesScreen, TerminalScreen};

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
        if self.is_connected_to_board == false {
            panels::home::render(ui, self);
        } else {
            panels::navigation::render(ui, self);
        }

        match self.app_state {
            DashboardScreen => panels::dashboard::render(ui, self),
            _ => {}
        }
    }
}
