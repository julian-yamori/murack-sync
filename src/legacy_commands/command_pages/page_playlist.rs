use std::sync::Arc;

use eframe::egui::{Ui, mutex::Mutex};

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    console::Console,
    egui_cui::CommandState,
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

    fn run_command(
        &mut self,
        console: Arc<Mutex<Console>>,
        _command_state: Arc<Mutex<CommandState>>,
    ) {
        // TODO: 実際のplaylist処理を実装

        let mut console = console.lock();

        console.add_log("[INFO] playlist コマンドを実行".to_string());
        console.add_log("[INFO] playlist 処理が完了しました".to_string());
    }
}
