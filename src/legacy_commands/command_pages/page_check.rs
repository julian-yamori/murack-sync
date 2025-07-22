use std::sync::Arc;

use eframe::egui::{Ui, mutex::Mutex};
use murack_core_app::command::CommandCheckArgs;
use sqlx::PgPool;

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    console::Console,
    di_registry::DIRegistry,
    egui_cui::CommandState,
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

    fn run_command(
        &mut self,
        console: Arc<Mutex<Console>>,
        command_state: Arc<Mutex<CommandState>>,
        di_registry: Arc<DIRegistry>,
        db_pool: Arc<PgPool>,
    ) {
        *command_state.lock() = CommandState::Running;

        // プロトタイプ処理を非同期で実行
        let target_path = self.target_path.clone();
        let ignore_dap_content = self.ignore_dap_content;
        let console_clone = console.clone();

        let command_state_clone = command_state.clone();

        tokio::spawn(async move {
            let command = di_registry.command_check(CommandCheckArgs {
                path: target_path.clone().into(),
                ignore_dap_content,
            });

            if let Err(e) = command.run(&db_pool).await {
                console_clone
                    .lock()
                    .add_error(format!("[ERROR] check 処理でエラーが発生しました: {e}"));
            }
            *command_state_clone.lock() = CommandState::NotRunning;
        });
    }
}
