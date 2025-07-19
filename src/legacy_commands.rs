mod command_pages;
mod console;
mod navigation;

use eframe::egui::{self, RichText};

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
                ui.add_space(4.0);

                // コマンドの簡易説明タイトル
                ui.label(RichText::new(page.page_discription()).heading().strong());

                ui.add_space(10.0);

                // パラメータの入力欄
                page.show_form(ui);

                ui.add_space(10.0);

                // 実行ボタン
                let button = ui.button(RichText::new("実行").heading());
                if button.clicked() {
                    page.run_command(&self.console);
                }

                ui.add_space(4.0);
            });

            ui.separator();

            // Console area
            ui.label("Console:");
            ui.add_space(5.0);
            self.console.show(ui);
        });
    }
}
