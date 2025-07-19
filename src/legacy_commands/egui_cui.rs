use std::sync::Arc;
use std::sync::mpsc::{self, Sender};

use anyhow::Result;
use eframe::egui::mutex::Mutex;

use crate::legacy_commands::console::Console;

/// コマンドの実行状態
#[derive(Debug, Default)]
pub enum CommandState {
    #[default]
    NotRunning,
    Running,
    Choice {
        available_choices: Vec<char>,
        message: String,
        choice_sender: Sender<char>,
    },
}

/// シンプルな Cui トレイト (プロトタイプ用)
pub trait SimpleCui {
    fn out_log(&self, message: &str);
    fn out_error(&self, message: &str);
    fn input_case(&self, cases: &[char], message: &str) -> Result<char>;
}

/// egui 用の Cui 実装
pub struct EguiCui {
    console: Arc<Mutex<Console>>,
    command_state: Arc<Mutex<CommandState>>,
}

impl EguiCui {
    pub fn new(console: Arc<Mutex<Console>>, command_state: Arc<Mutex<CommandState>>) -> Self {
        Self {
            console,
            command_state,
        }
    }
}

impl SimpleCui for EguiCui {
    fn out_log(&self, message: &str) {
        self.console.lock().add_log(message.to_string());
    }

    fn out_error(&self, message: &str) {
        self.console.lock().add_error(message.to_string());
    }

    fn input_case(&self, cases: &[char], message: &str) -> Result<char> {
        let (choice_sender, choice_receiver) = mpsc::channel();

        // 選択肢状態を設定
        *self.command_state.lock() = CommandState::Choice {
            available_choices: cases.to_vec(),
            message: message.to_string(),
            choice_sender,
        };

        // 選択されるまで待機
        let choice = choice_receiver.recv()?;

        // UI に選択終了を通知
        *self.command_state.lock() = CommandState::Running;

        Ok(choice)
    }
}
