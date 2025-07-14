#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Ui};
use std::collections::VecDeque;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Murack Sync",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Setup Japanese font
            setup_fonts(&cc.egui_ctx);

            Ok(Box::<MurackSyncApp>::default())
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Add Japanese font
    fonts.font_data.insert(
        "noto_sans_cjk".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/NotoSansCJK-Regular.otf")).into(),
    );

    // Add Japanese font to default font families
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_cjk".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "noto_sans_cjk".to_owned());

    // Apply font settings
    ctx.set_fonts(fonts);
}

#[derive(Clone)]
enum MessageType {
    Log,
    Error,
}

#[derive(Clone)]
struct Message {
    message_type: MessageType,
    text: String,
}

#[derive(Default)]
struct Console {
    messages: VecDeque<Message>,
}

impl Console {
    fn add_log(&mut self, text: String) {
        self.messages.push_back(Message {
            message_type: MessageType::Log,
            text,
        });
        // Keep only last 1000 messages
        if self.messages.len() > 1000 {
            self.messages.pop_front();
        }
    }

    fn add_error(&mut self, text: String) {
        self.messages.push_back(Message {
            message_type: MessageType::Error,
            text,
        });
        // Keep only last 1000 messages
        if self.messages.len() > 1000 {
            self.messages.pop_front();
        }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    for message in &self.messages {
                        let color = match message.message_type {
                            MessageType::Log => egui::Color32::LIGHT_GRAY,
                            MessageType::Error => egui::Color32::LIGHT_RED,
                        };
                        ui.colored_label(color, &message.text);
                    }
                });
            });
    }
}

struct HeaderForm {
    command_description: String,
}

impl HeaderForm {
    fn new(command_description: String) -> Self {
        Self {
            command_description,
        }
    }

    fn show<F>(&self, ui: &mut egui::Ui, content: F, on_run: Option<&dyn Fn()>)
    where
        F: FnOnce(&mut egui::Ui),
    {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(&self.command_description);
            ui.add_space(10.0);

            content(ui);

            ui.add_space(10.0);
            if ui.button("実行").clicked() {
                if let Some(callback) = on_run {
                    callback();
                }
            }
        });
    }
}

struct PageAdd {
    songs_path: String,
    header_form: HeaderForm,
}

impl Default for PageAdd {
    fn default() -> Self {
        Self {
            songs_path: String::new(),
            header_form: HeaderForm::new("曲をライブラリに追加".to_string()),
        }
    }
}

impl PageAdd {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(&self.header_form.command_description);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("追加する曲のライブラリパス:");
                ui.text_edit_singleline(&mut self.songs_path);
            });

            ui.add_space(10.0);
            ui.button("実行").clicked()
        })
        .inner
    }
}

#[derive(Default, PartialEq)]
enum CommandTab {
    #[default]
    Add,
    Playlist,
    Move,
}

struct PagePlaylist {
    header_form: HeaderForm,
}

impl Default for PagePlaylist {
    fn default() -> Self {
        Self {
            header_form: HeaderForm::new("DAPのプレイリストを更新".to_string()),
        }
    }
}

impl PagePlaylist {
    fn show(&mut self, ui: &mut Ui) -> bool {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(&self.header_form.command_description);
            ui.add_space(10.0);

            ui.label("プレイリストを更新します（入力不要）");

            ui.add_space(10.0);
            ui.button("実行").clicked()
        })
        .inner
    }
}

struct PageMove {
    src_path: String,
    dest_path: String,
    header_form: HeaderForm,
}

impl Default for PageMove {
    fn default() -> Self {
        Self {
            src_path: String::new(),
            dest_path: String::new(),
            header_form: HeaderForm::new("ライブラリ内で曲のパスを移動".to_string()),
        }
    }
}

impl PageMove {
    fn show(&mut self, ui: &mut Ui) -> bool {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(&self.header_form.command_description);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("移動元のライブラリパス:");
                ui.text_edit_singleline(&mut self.src_path);
            });

            ui.horizontal(|ui| {
                ui.label("移動先のライブラリパス:");
                ui.text_edit_singleline(&mut self.dest_path);
            });

            ui.add_space(10.0);
            ui.button("実行").clicked()
        })
        .inner
    }
}

#[derive(Default)]
struct MurackSyncApp {
    console: Console,
    page_add: PageAdd,
    page_playlist: PagePlaylist,
    page_move: PageMove,
    current_tab: CommandTab,
}

/// add ページの実行ボタン押下時
fn on_add_button(app: &mut MurackSyncApp, ui: &mut Ui) {
    // TODO: 実際のadd処理を実装

    if app.page_add.show(ui) {
        let path = &app.page_add.songs_path;
        if path.is_empty() {
            app.console
                .add_error("[ERROR] 追加する曲のパスが未入力です".to_owned());
            return;
        }
        app.console
            .add_log(format!("[INFO] add コマンドを実行: {path}"));
        app.console
            .add_log("[INFO] add 処理が完了しました".to_string());
    }
}

impl eframe::App for MurackSyncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Tab navigation
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, CommandTab::Add, "add");
                    ui.selectable_value(&mut self.current_tab, CommandTab::Playlist, "playlist");
                    ui.selectable_value(&mut self.current_tab, CommandTab::Move, "move");
                });

                ui.separator();

                // Header area for command input
                ui.allocate_ui_with_layout(
                    [ui.available_width(), 200.0].into(),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        match self.current_tab {
                            CommandTab::Add => on_add_button(self, ui),
                            CommandTab::Playlist => {
                                // TODO: 実際のadd処理を実装
                                if self.page_playlist.show(ui) {
                                    self.console
                                        .add_log("[INFO] playlist コマンドを実行".to_string());
                                    self.console
                                        .add_log("[INFO] playlist 処理が完了しました".to_string());
                                }
                            }
                            CommandTab::Move => {
                                // TODO: 実際のadd処理を実装
                                if self.page_move.show(ui) {
                                    let src_path = &self.page_move.src_path;
                                    let dest_path = &self.page_move.dest_path;
                                    if src_path.is_empty() || dest_path.is_empty() {
                                        self.console.add_error(
                                            "[ERROR] 移動元または移動先のパスが未入力です"
                                                .to_owned(),
                                        );
                                        return;
                                    }
                                    self.console.add_log(format!(
                                        "[INFO] move コマンドを実行: {} -> {}",
                                        src_path, dest_path
                                    ));
                                    self.console
                                        .add_log("[INFO] move 処理が完了しました".to_string());
                                }
                            }
                        }
                    },
                );

                ui.separator();

                // Console area
                ui.allocate_ui_with_layout(
                    [ui.available_width(), ui.available_height()].into(),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.label("Console:");
                        ui.add_space(5.0);

                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(34, 34, 34))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                            .inner_margin(egui::Margin::same(8))
                            .show(ui, |ui| {
                                self.console.show(ui);
                            });
                    },
                );
            });
        });
    }
}
