use std::fmt::Arguments;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender};

use anyhow::Result;
use eframe::egui::mutex::Mutex;
use murack_core_app::cui::Cui;

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

impl Cui for EguiCui {
    fn out(&self, args: Arguments) -> anyhow::Result<()> {
        // ほんとは改行せず連結すべきだけど、改行なしの追加ができる設計になってないのでとりあえず
        self.outln(args)
    }

    fn outln(&self, args: Arguments) -> anyhow::Result<()> {
        self.console.lock().add_log(args.to_string());

        Ok(())
    }

    fn err(&self, args: Arguments) -> anyhow::Result<()> {
        // ほんとは改行せず連結すべきだけど、改行なしの追加ができる設計になってないのでとりあえず
        self.errln(args)
    }

    fn errln(&self, args: Arguments) -> anyhow::Result<()> {
        self.console.lock().add_error(args.to_string());

        Ok(())
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
