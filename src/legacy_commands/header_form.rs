use eframe::egui;

pub struct HeaderForm {
    // TODO: 暫定で pub
    pub command_description: String,
}

impl HeaderForm {
    pub fn new(command_description: String) -> Self {
        Self {
            command_description,
        }
    }

    pub fn show<F>(&self, ui: &mut egui::Ui, content: F, on_run: Option<&dyn Fn()>)
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
