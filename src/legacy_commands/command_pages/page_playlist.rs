use std::sync::Arc;

use eframe::egui::Ui;
use tokio::task::JoinHandle;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    di_registry::DIRegistry,
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

    fn run_command(&mut self, di_registry: Arc<DIRegistry>) -> JoinHandle<anyhow::Result<()>> {
        tokio::spawn(async move {
            let console = di_registry.console();
            let mut console = console.lock();

            // TODO: 実際のplaylist処理を実装

            console.add_log("[INFO] playlist コマンドを実行".to_string());
            console.add_log("[INFO] playlist 処理が完了しました".to_string());

            Ok(())
        })
    }
}
