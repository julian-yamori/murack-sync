use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

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

pub struct Console {
    messages: Arc<Mutex<VecDeque<Message>>>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

impl Clone for Console {
    fn clone(&self) -> Self {
        Self {
            messages: Arc::clone(&self.messages),
        }
    }
}

impl Console {
    pub fn add_log(&self, text: String) {
        if let Ok(mut messages) = self.messages.lock() {
            messages.push_back(Message {
                message_type: MessageType::Log,
                text,
            });
            // Keep only last 1000 messages
            if messages.len() > 1000 {
                messages.pop_front();
            }
        }
    }

    pub fn add_error(&self, text: String) {
        if let Ok(mut messages) = self.messages.lock() {
            messages.push_back(Message {
                message_type: MessageType::Error,
                text,
            });
            // Keep only last 1000 messages
            if messages.len() > 1000 {
                messages.pop_front();
            }
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(34, 34, 34))
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Ok(messages) = self.messages.lock() {
                            for message in messages.iter() {
                                let color = message.message_type.color();
                                ui.colored_label(color, &message.text);
                            }
                        }
                    });
            });
    }
}
