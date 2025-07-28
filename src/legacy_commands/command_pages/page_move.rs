use std::sync::Arc;

use anyhow::anyhow;
use eframe::egui::Ui;
use murack_core_app::command::CommandMoveArgs;
use murack_core_domain::{EmptyStringError, NonEmptyString};
use tokio::task::JoinHandle;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    di_registry::DIRegistry,
};

/// move コマンドのページ
#[derive(Default)]
pub struct PageMove {
    src_path: String,
    dest_path: String,
}

impl CommandPage for PageMove {
    fn page_type(&self) -> PageType {
        PageType::Move
    }

    fn page_discription(&self) -> &str {
        "ライブラリ内で曲のパスを移動"
    }

    fn show_form(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("移動元のライブラリパス:");
            ui.text_edit_singleline(&mut self.src_path);
        });

        ui.horizontal(|ui| {
            ui.label("移動先のライブラリパス:");
            ui.text_edit_singleline(&mut self.dest_path);
        });
    }

    fn run_command(&mut self, di_registry: Arc<DIRegistry>) -> JoinHandle<anyhow::Result<()>> {
        let src_path = self.src_path.clone();
        let dest_path = self.dest_path.clone();

        tokio::spawn(async move {
            let src_path: NonEmptyString = match src_path.try_into() {
                Ok(s) => s,
                Err(EmptyStringError) => {
                    return Err(anyhow!("移動元のパスが未入力です"));
                }
            };

            let dest_path: NonEmptyString = match dest_path.try_into() {
                Ok(s) => s,
                Err(EmptyStringError) => {
                    return Err(anyhow!("移動先のパスが未入力です"));
                }
            };

            let command = di_registry.command_move(CommandMoveArgs {
                src_path,
                dest_path,
            });
            let db_pool = di_registry.db_pool();

            command.run(&db_pool).await
        })
    }
}
