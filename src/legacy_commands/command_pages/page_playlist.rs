use std::sync::{Arc, Mutex};

use eframe::egui::Ui;

use crate::legacy_commands::{
    console::Console,
    navigation::{CommandPage, PageType},
};

/// playlist コマンドのページ
#[derive(Default)]
pub struct PagePlaylist;

impl CommandPage for PagePlaylist {
    fn page_type(&self) -> PageType {
        PageType::Playlist
    }

    fn page_discription(&self) -> &str {
        "DAPのプレイリストを更新"
    }

    fn show_form(&mut self, _ui: &mut Ui) {}

    fn run_command(&mut self, console: Arc<Mutex<Console>>) {
        // TODO: 実際のplaylist処理を実装

        if let Ok(mut console) = console.lock() {
            console.add_log("[INFO] playlist コマンドを実行".to_string());
            console.add_log("[INFO] playlist 処理が完了しました".to_string());
        }
    }
}
