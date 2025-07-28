use std::sync::Arc;

use anyhow::anyhow;
use eframe::egui::Ui;
use murack_core_app::command::CommandRemoveArgs;
use murack_core_domain::{EmptyStringError, NonEmptyString};
use tokio::task::JoinHandle;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    di_registry::DIRegistry,
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

    fn run_command(&mut self, di_registry: Arc<DIRegistry>) -> JoinHandle<anyhow::Result<()>> {
        let target_path = self.target_path.clone();

        tokio::spawn(async move {
            let target_path: NonEmptyString = match target_path.try_into() {
                Ok(s) => s,
                Err(EmptyStringError) => return Err(anyhow!("削除する曲のパスが未入力です")),
            };

            let command = di_registry.command_remove(CommandRemoveArgs { path: target_path });
            let db_pool = di_registry.db_pool();

            command.run(&db_pool).await
        })
    }
}
