#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod config;
mod database;
mod legacy_commands;

use std::sync::Arc;

use eframe::egui;
use murack_core_app::Config;
use sqlx::PgPool;

use crate::legacy_commands::LegacyCommandsApp;

#[tokio::main]
async fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let config = config::load_config().map_err(|e| eframe::Error::AppCreation(e.into()))?;
    let config = Arc::new(config);

    // Connect to database
    let db_pool = database::connect_db_pool(&config.database_url)
        .await
        .map_err(|e| eframe::Error::AppCreation(e.into()))?;
    let db_pool = Arc::new(db_pool);

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

            cc.egui_ctx.set_fonts(font_definitions());

            Ok(Box::new(MurackSyncApp {
                legacy_commands_app: LegacyCommandsApp::new(config.clone(), db_pool.clone()),
                _config: config,
                _db_pool: db_pool,
            }))
        }),
    )
}

fn font_definitions() -> egui::FontDefinitions {
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

    fonts
}

struct MurackSyncApp {
    // legacy commands 以外の正式版の機能で使う予定
    _config: Arc<Config>,
    _db_pool: Arc<PgPool>,

    legacy_commands_app: LegacyCommandsApp,
}

impl eframe::App for MurackSyncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.legacy_commands_app.show(ui);
        });
    }
}
