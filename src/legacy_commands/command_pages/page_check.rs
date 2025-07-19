use std::sync::Arc;
use std::thread;
use std::time::Duration;

use eframe::egui::{Ui, mutex::Mutex};

use crate::legacy_commands::{
    command_pages::{CommandPage, PageType},
    console::Console,
    egui_cui::{CommandState, EguiCui, SimpleCui},
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
    ) {
        *command_state.lock() = CommandState::Running;

        // EguiCui を作成
        let egui_cui = EguiCui::new(console.clone(), command_state.clone());

        // プロトタイプ処理を非同期で実行
        let path = self.target_path.clone();
        let ignore_dap = self.ignore_dap_content;
        let console_clone = console.clone();

        let command_state_clone = command_state.clone();

        thread::spawn(move || {
            if let Err(e) = run_check_prototype(path, ignore_dap, egui_cui) {
                console_clone
                    .lock()
                    .add_error(format!("[ERROR] check 処理でエラーが発生しました: {e}"));
            }
            *command_state_clone.lock() = CommandState::NotRunning;
        });
    }
}

/// check コマンドのプロトタイプ実装
fn run_check_prototype(path: String, ignore_dap: bool, cui: EguiCui) -> anyhow::Result<()> {
    cui.out_log("====================");
    cui.out_log("check コマンドを開始します");
    cui.out_log(&format!(
        "対象パス: {}",
        if path.is_empty() {
            "(ルート)"
        } else {
            &path
        }
    ));
    cui.out_log(&format!("DAPファイル内容無視: {ignore_dap}"));
    cui.out_log("====================");

    // ファイルリストアップのシミュレーション
    cui.out_log("PCの検索中...");
    thread::sleep(Duration::from_millis(500));
    cui.out_log("DAPの検索中...");
    thread::sleep(Duration::from_millis(500));
    cui.out_log("DBの検索中...");
    thread::sleep(Duration::from_millis(500));

    // 簡易チェックのシミュレーション
    cui.out_log("チェック中...(1/100)");
    thread::sleep(Duration::from_millis(300));
    cui.out_log("チェック中...(50/100)");
    thread::sleep(Duration::from_millis(300));
    cui.out_log("チェック中...(100/100)");

    // 問題検出のシミュレーション
    cui.out_log("");
    cui.out_log("# music/album1/song1.flac");
    cui.out_error("---- DAPに存在しません");
    cui.out_log("# music/album2/song2.flac");
    cui.out_error("---- PCとDBで再生時間が異なります");
    cui.out_log("");

    cui.out_log("2個のファイルで問題を検出しました。");

    // 継続確認
    let continue_choice = cui.input_case(&['y', 'n'], "解決処理を行いますか? (y/n)->")?;
    if continue_choice == 'n' {
        cui.out_log("処理を中止しました。");
        return Ok(());
    }

    // 個別解決のシミュレーション
    cui.out_log("====================");
    cui.out_log("music/album1/song1.flac");
    cui.out_log("(1/2)");
    cui.out_log("");
    cui.out_log("---- DAPに存在しません");
    cui.out_log("");

    let choice1 = cui.input_case(&['1', '0', '-'], "処理を選択してください ->")?;

    match choice1 {
        '1' => {
            cui.out_log("[INFO] PCからDAPへコピーしました");
        }
        '0' => {
            cui.out_log("[INFO] 解決をスキップしました");
        }
        '-' => {
            cui.out_log("[INFO] 解決処理を中止しました");
            return Ok(());
        }
        _ => unreachable!(),
    }

    cui.out_log("");
    cui.out_log("====================");
    cui.out_log("music/album2/song2.flac");
    cui.out_log("(2/2)");
    cui.out_log("");
    cui.out_log("---- 再生時間: 245000ms | 244500ms");
    cui.out_log("PC vs DB");
    cui.out_log("");

    let choice2 = cui.input_case(&['1', '0', '-'], "処理を選択してください ->")?;

    match choice2 {
        '1' => {
            cui.out_log("[INFO] PCからDBへ上書きしました");
        }
        '0' => {
            cui.out_log("[INFO] 解決をスキップしました");
        }
        '-' => {
            cui.out_log("[INFO] 解決処理を中止しました");
            return Ok(());
        }
        _ => unreachable!(),
    }

    cui.out_log("");
    cui.out_log("====================");
    cui.out_log("全ての問題の解決処理が終了しました。");

    Ok(())
}
