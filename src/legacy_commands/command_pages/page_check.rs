use std::sync::Arc;

use eframe::egui::Ui;
use murack_core_app::command::CommandCheckArgs;
use murack_core_domain::NonEmptyString;
use tokio::task::JoinHandle;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    di_registry::DIRegistry,
};

/// check コマンドのページ
#[derive(Default)]
pub struct PageCheck {
    target_path: String,
    ignore_dap_content: bool,
}

impl CommandPage for PageCheck {
    fn page_type(&self) -> PageType {
        PageType::Check
    }

    fn page_discription(&self) -> &str {
        "PC・DAP・DBの齟齬を確認・解決"
    }

    fn show_form(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("確認対象のライブラリパス:");
            ui.text_edit_singleline(&mut self.target_path);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.ignore_dap_content, "DAPファイル内容を無視 (-i)");
        });
    }

    fn run_command(&mut self, di_registry: Arc<DIRegistry>) -> JoinHandle<anyhow::Result<()>> {
        let target_path: Option<NonEmptyString> = self.target_path.clone().try_into().ok();
        let ignore_dap_content = self.ignore_dap_content;

        tokio::spawn(async move {
            let command = di_registry.command_check(CommandCheckArgs {
                path: target_path,
                ignore_dap_content,
            });
            let db_pool = di_registry.db_pool();

            command.run(&db_pool).await
        })
    }
}
