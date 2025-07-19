use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::Result;

use crate::legacy_commands::console::Console;

/// 選択肢の状態管理
#[derive(Debug)]
pub struct ChoiceState {
    pub available_choices: Vec<char>,
    pub selected_choice: Option<char>,
    pub waiting_for_input: bool,
    pub message: String,
}

impl Default for ChoiceState {
    fn default() -> Self {
        Self {
            available_choices: Vec::new(),
            selected_choice: None,
            waiting_for_input: false,
            message: String::new(),
        }
    }
}

/// シンプルな Cui トレイト (プロトタイプ用)
pub trait SimpleCui {
    fn out_log(&self, message: &str);
    fn out_error(&self, message: &str);
    fn input_case(&self, cases: &[char], message: &str) -> Result<char>;
}

/// egui 用の Cui 実装
#[derive(Clone)]
pub struct EguiCui {
    console: Console,
    choice_state: Arc<Mutex<ChoiceState>>,
}

impl EguiCui {
    pub fn new(console: Console) -> Self {
        Self {
            console,
            choice_state: Arc::new(Mutex::new(ChoiceState::default())),
        }
    }

    /// 選択肢状態への参照を取得
    pub fn choice_state(&self) -> Arc<Mutex<ChoiceState>> {
        Arc::clone(&self.choice_state)
    }

    /// 選択肢を設定
    pub fn set_choice(&self, choice: char) {
        if let Ok(mut state) = self.choice_state.lock() {
            state.selected_choice = Some(choice);
        }
    }
}

impl SimpleCui for EguiCui {
    fn out_log(&self, message: &str) {
        self.console.add_log(message.to_string());
    }

    fn out_error(&self, message: &str) {
        self.console.add_error(message.to_string());
    }

    fn input_case(&self, cases: &[char], message: &str) -> Result<char> {
        // 選択肢状態を設定
        {
            let mut state = self.choice_state.lock().unwrap();
            state.available_choices = cases.to_vec();
            state.selected_choice = None;
            state.waiting_for_input = true;
            state.message = message.to_string();
        }

        // 選択されるまで待機
        loop {
            {
                let state = self.choice_state.lock().unwrap();
                if let Some(choice) = state.selected_choice {
                    // 選択肢をリセット
                    drop(state);
                    let mut state = self.choice_state.lock().unwrap();
                    state.waiting_for_input = false;
                    return Ok(choice);
                }
            }

            // 短時間スリープして CPU 使用率を下げる
            thread::sleep(Duration::from_millis(50));
        }
    }
}
