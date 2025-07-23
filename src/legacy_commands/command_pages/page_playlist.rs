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
            let command = di_registry.command_playlist();
            let db_pool = di_registry.db_pool();

            command.run(&db_pool).await
        })
    }
}
