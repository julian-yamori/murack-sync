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

    fn show_form(&mut self, ui: &mut Ui) {
        ui.label("プレイリストを更新します（入力不要）");
    }

    fn run_command(&mut self, console: &mut Console) {
        // TODO: 実際のplaylist処理を実装

        console.add_log("[INFO] playlist コマンドを実行".to_string());
        console.add_log("[INFO] playlist 処理が完了しました".to_string());
    }
}
