use std::sync::Arc;

use anyhow::anyhow;
use eframe::egui::Ui;
use murack_core_app::command::CommandAddArgs;
use murack_core_domain::{EmptyStringError, NonEmptyString};
use tokio::task::JoinHandle;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    di_registry::DIRegistry,
};

/// add コマンドのページ
#[derive(Default)]
pub struct PageAdd {
    tracks_path: String,
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
            ui.text_edit_singleline(&mut self.tracks_path);
        });
    }

    fn run_command(&mut self, di_registry: Arc<DIRegistry>) -> JoinHandle<anyhow::Result<()>> {
        let tracks_path = self.tracks_path.clone();

        tokio::spawn(async move {
            let tracks_path: NonEmptyString = match tracks_path.try_into() {
                Ok(s) => s,
                Err(EmptyStringError) => return Err(anyhow!("追加する曲のパスが未入力です")),
            };

            let command = di_registry.command_add(CommandAddArgs { path: tracks_path });
            let db_pool = di_registry.db_pool();

            command.run(&db_pool).await
        })
    }
}
