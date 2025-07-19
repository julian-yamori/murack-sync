use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex};

use crate::legacy_commands::{console::Console, egui_cui::CommandState};

#[derive(PartialEq, Clone, Copy)]
pub enum PageType {
    Add,
    Playlist,
    Move,
    Remove,
    Check,
}

/// レガシーコマンド 1 つを扱うページの抽象化 trait
pub trait CommandPage {
    fn page_type(&self) -> PageType;

    fn page_discription(&self) -> &str;

    fn show_form(&mut self, ui: &mut egui::Ui);

    fn run_command(
        &mut self,
        console: Arc<Mutex<Console>>,
        command_state: Arc<Mutex<CommandState>>,
    );
}
