use std::sync::Arc;
use std::sync::mpsc::{self, Sender};

use anyhow::Result;
use eframe::egui::mutex::Mutex;

use crate::legacy_commands::console::Console;

/// 選択肢の状態管理
#[derive(Debug)]
pub struct ChoiceState {
    pub available_choices: Vec<char>,
    pub message: String,
    pub choice_sender: Sender<char>,
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
    choice_state: Arc<Mutex<Option<ChoiceState>>>,
}

impl EguiCui {
    pub fn new(console: Arc<Mutex<Console>>) -> Self {
        Self {
            console,
            choice_state: Arc::new(Mutex::new(None)),
        }
    }

    pub fn choice_state(&self) -> Arc<Mutex<Option<ChoiceState>>> {
        self.choice_state.clone()
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
        *self.choice_state.lock() = Some(ChoiceState {
            available_choices: cases.to_vec(),
            message: message.to_string(),
            choice_sender,
        });

        // 選択されるまで待機
        let choice = choice_receiver.recv()?;

        // UI に選択終了を通知
        *self.choice_state.lock() = None;

        Ok(choice)
    }
}
