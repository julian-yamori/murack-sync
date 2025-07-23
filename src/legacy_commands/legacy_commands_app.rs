use std::sync::Arc;

use eframe::egui::{self, RichText, mutex::Mutex};
use murack_core_app::Config;
use sqlx::PgPool;

use crate::legacy_commands::{
    command_pages::CommandPage, console::Console, di_registry::DIRegistry, egui_cui::CommandState,
    navigation::LegacyCommandsNavigation,
};

pub struct LegacyCommandsApp {
    console: Arc<Mutex<Console>>,
    di_registry: Arc<DIRegistry>,
    navigation: LegacyCommandsNavigation,
    command_state: Arc<Mutex<CommandState>>,
}

impl LegacyCommandsApp {
    pub fn new(config: Arc<Config>, db_pool: Arc<PgPool>) -> Self {
        let console = Arc::<Mutex<Console>>::default();
        let command_state = Arc::<Mutex<CommandState>>::default();
        let di_registry = DIRegistry::new(console.clone(), command_state.clone(), config, db_pool);

        Self {
            di_registry: Arc::new(di_registry),
            navigation: LegacyCommandsNavigation::default(),
            console,
            command_state,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let command_running = !matches!(&*self.command_state.lock(), CommandState::NotRunning);

        ui.vertical(|ui| {
            self.navigation.show_tab(ui, !command_running);
            let page = &mut *self.navigation.current_page;

            ui.separator();

            ui.vertical_centered(|ui| {
                if command_running {
                    ui.disable();
                }

                ui.add_space(4.0);

                // コマンドの簡易説明タイトル
                ui.label(RichText::new(page.page_discription()).heading().strong());

                ui.add_space(10.0);

                // パラメータの入力欄
                page.show_form(ui);

                ui.add_space(10.0);

                // 実行ボタン
                let button = ui.button(RichText::new("実行").heading());
                if button.clicked() {
                    run_command(
                        page,
                        self.console.clone(),
                        self.command_state.clone(),
                        self.di_registry.clone(),
                    );
                }

                ui.add_space(4.0);
            });

            // 選択肢が待機中なら表示
            if let CommandState::Choice {
                available_choices,
                message,
                choice_sender,
            } = &*self.command_state.lock()
            {
                ui.separator();
                ui.label(message);

                ui.horizontal(|ui| {
                    for &choice in available_choices {
                        let button_text = match choice {
                            '1' => "1: PCからDBへ上書き",
                            '2' => "2: DBからPCへ上書き",
                            '0' => "0: 解決せずに次へ",
                            '-' => "-: 解決処理を中止",
                            _ => &format!("{choice}: その他"),
                        };

                        if ui.button(button_text).clicked() {
                            if let Err(e) = choice_sender.send(choice) {
                                println!("{e}");
                            }
                        }
                    }
                });
            }

            ui.separator();

            // Console area
            ui.label("Console:");
            ui.add_space(5.0);
            self.console.lock().show(ui);
        });
    }
}

fn run_command(
    current_page: &mut dyn CommandPage,
    console: Arc<Mutex<Console>>,
    command_state: Arc<Mutex<CommandState>>,
    di_registry: Arc<DIRegistry>,
) {
    *command_state.lock() = CommandState::Running;

    let command_handle = current_page.run_command(di_registry.clone());

    tokio::spawn(async move {
        let command_result = command_handle.await;

        *command_state.lock() = CommandState::NotRunning;

        if let Err(e) = command_result {
            console.lock().add_error(e.to_string());
        }
    });
}
