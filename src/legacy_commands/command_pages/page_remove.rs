use std::sync::{Arc, Mutex};

use eframe::egui::Ui;

use crate::legacy_commands::{
    console::Console,
    navigation::{CommandPage, PageType},
};

/// remove コマンドのページ
#[derive(Default)]
pub struct PageRemove {
    target_path: String,
}

impl CommandPage for PageRemove {
    fn page_type(&self) -> PageType {
        PageType::Remove
    }

    fn page_discription(&self) -> &str {
        "ライブラリから曲を削除"
    }

    fn show_form(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("削除する曲のライブラリパス:");
            ui.text_edit_singleline(&mut self.target_path);
        });
    }

    fn run_command(&mut self, console: Arc<Mutex<Console>>) {
        // TODO: 実際のremove処理を実装

        if let Ok(mut console) = console.lock() {
            let path = &self.target_path;
            if path.is_empty() {
                console.add_error("[ERROR] 削除する曲のパスが未入力です".to_owned());
                return;
            }
            console.add_log(format!("[INFO] remove コマンドを実行: {path}"));
            console.add_log("[INFO] remove 処理が完了しました".to_string());
        }
    }
}
