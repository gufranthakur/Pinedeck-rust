use eframe::egui::{self};

use crate::core::window::{AppState::DashboardScreen, PineDeck};

pub fn render(ui: &mut egui::Ui, pinedeck: &mut PineDeck) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("PineDeck")
                    .color(egui::Color32::WHITE)
                    .size(42.0)
                    .strong(),
            );

            ui.label(
                egui::RichText::new("Please connect to a board to continue")
                    .color(egui::Color32::LIGHT_GRAY)
                    .size(24.0)
                    .strong(),
            );

            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new("Search")
                            .color(egui::Color32::WHITE)
                            .size(18.0),
                    )
                    .fill(egui::Color32::DARK_GREEN)
                    .corner_radius(egui::CornerRadius::same(1))
                    .min_size(egui::Vec2 { x: 80.0, y: 25.0 }),
                )
                .clicked()
            {
                pinedeck.app_state = DashboardScreen;
            }
        })
    });
}
