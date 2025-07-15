mod command_pages;
mod console;
mod header_form;

use eframe::egui;

use crate::legacy_commands::{
    command_pages::{PageAdd, PageMove, PagePlaylist},
    console::Console,
};

#[derive(Default)]
pub struct LegacyCommandsApp {
    console: Console,
    page_add: PageAdd,
    page_playlist: PagePlaylist,
    page_move: PageMove,
    current_tab: CommandTab,
}

impl LegacyCommandsApp {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Tab navigation
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, CommandTab::Add, "add");
                ui.selectable_value(&mut self.current_tab, CommandTab::Playlist, "playlist");
                ui.selectable_value(&mut self.current_tab, CommandTab::Move, "move");
            });

            ui.separator();

            // Header area for command input
            ui.allocate_ui_with_layout(
                [ui.available_width(), 200.0].into(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| match self.current_tab {
                    CommandTab::Add => self.page_add.show(&mut self.console, ui),
                    CommandTab::Playlist => self.page_playlist.show(&mut self.console, ui),
                    CommandTab::Move => self.page_move.show(&mut self.console, ui),
                },
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

#[derive(Default, PartialEq)]
enum CommandTab {
    #[default]
    Add,
    Playlist,
    Move,
}
