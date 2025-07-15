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
            let page = &mut *self.navigation.current_page;

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(page.page_discription());
                ui.add_space(10.0);

                page.show_form(ui);

                ui.add_space(10.0);

                let button = ui.button("実行");
                if button.clicked() {
                    page.run_command(&mut self.console);
                }
            });

            ui.separator();

            // Console area
            ui.label("Console:");
            ui.add_space(5.0);
            ui.allocate_ui(ui.available_size(), |ui| {
                self.console.show(ui);
            });
        });
    }
}
