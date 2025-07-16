use eframe::egui::{self, RichText};

use crate::legacy_commands::{
    command_pages::{PageAdd, PageMove, PagePlaylist},
    console::Console,
};

pub struct LegacyCommandsNavigation {
    pub current_page: Box<dyn CommandPage>,
}

impl LegacyCommandsNavigation {
    pub fn show_tab(&mut self, ui: &mut egui::Ui) {
        let old_type = self.current_page.page_type();
        let mut current_type = old_type;

        ui.horizontal(|ui| {
            ui.selectable_value(&mut current_type, PageType::Add, button_text("add"));
            ui.selectable_value(
                &mut current_type,
                PageType::Playlist,
                button_text("playlist"),
            );
            ui.selectable_value(&mut current_type, PageType::Move, button_text("move"));
        });

        if old_type != current_type {
            self.current_page = current_type.build_default_page();
        }
    }
}

fn button_text(text: &str) -> RichText {
    RichText::new(text).heading()
}

impl Default for LegacyCommandsNavigation {
    fn default() -> Self {
        Self {
            current_page: Box::new(PageAdd::default()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PageType {
    Add,
    Playlist,
    Move,
}

impl PageType {
    fn build_default_page(&self) -> Box<dyn CommandPage> {
        match self {
            PageType::Add => Box::new(PageAdd::default()),
            PageType::Playlist => Box::new(PagePlaylist {}),
            PageType::Move => Box::new(PageMove::default()),
        }
    }
}

pub trait CommandPage {
    fn page_type(&self) -> PageType;

    fn page_discription(&self) -> &str;

    fn show_form(&mut self, ui: &mut egui::Ui);

    fn run_command(&mut self, console: &mut Console);
}
