use std::sync::Arc;

use anyhow::anyhow;
use eframe::egui::Ui;
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
            if src_path.is_empty() || dest_path.is_empty() {
                return Err(anyhow!("移動元または移動先のパスが未入力です"));
            }

            // TODO: 実際のmove処理を実装

            let console = di_registry.console();
            let mut console = console.lock();

            console.add_log(format!(
                "[INFO] move コマンドを実行: {src_path} -> {dest_path}"
            ));
            console.add_log("[INFO] move 処理が完了しました".to_string());

            Ok(())
        })
    }
}
