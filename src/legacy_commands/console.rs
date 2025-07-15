use std::collections::VecDeque;

use eframe::egui::{self};

#[derive(Clone)]
enum MessageType {
    Log,
    Error,
}

impl MessageType {
    pub fn color(&self) -> egui::Color32 {
        match self {
            MessageType::Log => egui::Color32::LIGHT_GRAY,
            MessageType::Error => egui::Color32::LIGHT_RED,
        }
    }
}

#[derive(Clone)]
struct Message {
    message_type: MessageType,
    text: String,
}

#[derive(Default)]
pub struct Console {
    messages: VecDeque<Message>,
}

impl Console {
    pub fn add_log(&mut self, text: String) {
        self.messages.push_back(Message {
            message_type: MessageType::Log,
            text,
        });
        // Keep only last 1000 messages
        if self.messages.len() > 1000 {
            self.messages.pop_front();
        }
    }

    pub fn add_error(&mut self, text: String) {
        self.messages.push_back(Message {
            message_type: MessageType::Error,
            text,
        });
        // Keep only last 1000 messages
        if self.messages.len() > 1000 {
            self.messages.pop_front();
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    for message in &self.messages {
                        let color = message.message_type.color();
                        ui.colored_label(color, &message.text);
                    }
                });
            });
    }
}
