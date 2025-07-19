use std::sync::Arc;

use eframe::egui::{Ui, mutex::Mutex};

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    console::Console,
    egui_cui::CommandState,
};

/// add コマンドのページ
#[derive(Default)]
pub struct PageAdd {
    songs_path: String,
}

impl CommandPage for PageAdd {
    fn page_type(&self) -> PageType {
        PageType::Add
    }

    fn page_discription(&self) -> &str {
        "曲をライブラリに追加"
    }

    fn show_form(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("追加する曲のライブラリパス:");
            ui.text_edit_singleline(&mut self.songs_path);
        });
    }

    fn run_command(
        &mut self,
        console: Arc<Mutex<Console>>,
        _command_state: Arc<Mutex<CommandState>>,
    ) {
        // TODO: 実際のadd処理を実装

        let mut console = console.lock();

        let path = &self.songs_path;
        if path.is_empty() {
            console.add_error("[ERROR] 追加する曲のパスが未入力です".to_owned());
            return;
        }
        console.add_log(format!("[INFO] add コマンドを実行: {path}"));
        console.add_log("[INFO] add 処理が完了しました".to_string());
    }
}
