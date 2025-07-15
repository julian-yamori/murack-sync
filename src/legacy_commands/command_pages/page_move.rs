use eframe::egui::Ui;

use crate::legacy_commands::{
    console::Console,
    navigation::{CommandPage, PageType},
};

#[derive(Default)]
pub struct PageMove {
    src_path: String,
    dest_path: String,
}

impl PageMove {
    fn run_command(&self, console: &mut Console) {
        // TODO: 実際のmove処理を実装

        let src_path = &self.src_path;
        let dest_path = &self.dest_path;
        if src_path.is_empty() || dest_path.is_empty() {
            console.add_error("[ERROR] 移動元または移動先のパスが未入力です".to_owned());
            return;
        }
        console.add_log(format!(
            "[INFO] move コマンドを実行: {src_path} -> {dest_path}"
        ));
        console.add_log("[INFO] move 処理が完了しました".to_string());
    }
}

impl CommandPage for PageMove {
    fn page_type(&self) -> PageType {
        PageType::Move
    }

    fn show(&mut self, console: &mut Console, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label("ライブラリ内で曲のパスを移動");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("移動元のライブラリパス:");
                ui.text_edit_singleline(&mut self.src_path);
            });

            ui.horizontal(|ui| {
                ui.label("移動先のライブラリパス:");
                ui.text_edit_singleline(&mut self.dest_path);
            });

            ui.add_space(10.0);

            let button = ui.button("実行");
            if button.clicked() {
                self.run_command(console);
            }
        });
    }
}
