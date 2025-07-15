use eframe::egui::Ui;

use crate::legacy_commands::{
    console::Console,
    navigation::{CommandPage, PageType},
};

#[derive(Default)]
pub struct PagePlaylist;

impl PagePlaylist {
    fn run_command(&self, console: &mut Console) {
        // TODO: 実際のplaylist処理を実装

        console.add_log("[INFO] playlist コマンドを実行".to_string());
        console.add_log("[INFO] playlist 処理が完了しました".to_string());
    }
}

impl CommandPage for PagePlaylist {
    fn page_type(&self) -> PageType {
        PageType::Playlist
    }

    fn show(&mut self, console: &mut Console, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label("DAPのプレイリストを更新");
            ui.add_space(10.0);

            ui.label("プレイリストを更新します（入力不要）");

            ui.add_space(10.0);

            let button = ui.button("実行");
            if button.clicked() {
                self.run_command(console);
            }
        });
    }
}
