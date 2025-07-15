mod command_pages;
mod console;
mod navigation;

use eframe::egui;

use crate::legacy_commands::{console::Console, navigation::LegacyCommandsNavigation};

#[derive(Default)]
pub struct LegacyCommandsApp {
    console: Console,
    navigation: LegacyCommandsNavigation,
}

impl LegacyCommandsApp {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.navigation.show_tab(ui);

            ui.separator();

            // Header area for command input
            ui.allocate_ui_with_layout(
                [ui.available_width(), 200.0].into(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| self.navigation.current_page.show(&mut self.console, ui),
            );

            ui.separator();

            // Console area
            ui.allocate_ui_with_layout(
                [ui.available_width(), ui.available_height()].into(),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.label("Console:");
                    ui.add_space(5.0);

                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(34, 34, 34))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            self.console.show(ui);
                        });
                },
            );
        });
    }
}
