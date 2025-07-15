use eframe::egui::Ui;

use crate::legacy_commands::{
    console::Console,
    navigation::{CommandPage, PageType},
};

#[derive(Default)]
pub struct PageAdd {
    songs_path: String,
}

impl PageAdd {
    fn run_command(&self, console: &mut Console) {
        // TODO: 実際のadd処理を実装

        let path = &self.songs_path;
        if path.is_empty() {
            console.add_error("[ERROR] 追加する曲のパスが未入力です".to_owned());
            return;
        }
        console.add_log(format!("[INFO] add コマンドを実行: {path}"));
        console.add_log("[INFO] add 処理が完了しました".to_string());
    }
}

impl CommandPage for PageAdd {
    fn page_type(&self) -> PageType {
        PageType::Add
    }

    fn show(&mut self, console: &mut Console, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label("曲をライブラリに追加");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("追加する曲のライブラリパス:");
                ui.text_edit_singleline(&mut self.songs_path);
            });

            ui.add_space(10.0);

            let button = ui.button("実行");
            if button.clicked() {
                self.run_command(console);
            }
        });
    }
}
