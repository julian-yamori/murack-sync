use eframe::egui::{self, RichText};

use crate::legacy_commands::command_pages::{
    CommandPage, PageAdd, PageCheck, PageMove, PagePlaylist, PageRemove, PageType,
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
            ui.selectable_value(&mut current_type, PageType::Remove, button_text("remove"));
            ui.selectable_value(&mut current_type, PageType::Check, button_text("check"));
        });

        if old_type != current_type {
            self.current_page = default_page_by_type(&current_type);
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

pub fn default_page_by_type(page_type: &PageType) -> Box<dyn CommandPage> {
    match page_type {
        PageType::Add => Box::new(PageAdd::default()),
        PageType::Playlist => Box::new(PagePlaylist {}),
        PageType::Move => Box::new(PageMove::default()),
        PageType::Remove => Box::new(PageRemove::default()),
        PageType::Check => Box::new(PageCheck::default()),
    }
}
