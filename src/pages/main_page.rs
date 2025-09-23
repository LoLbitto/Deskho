use crate::pages::Page;
use egui::Ui;

pub struct MainPage {}

impl Page for MainPage {
    fn update(&mut self, ui: &mut Ui) {
        ui.heading("teste pagina");
        ui.with_layout(egui::Layout::bottom_up(egui::Align::TOP), |ui| {
            ui.horizontal(|ui| {
                ui.set_row_height(50.0);
                let frame = egui::Frame::none().fill(egui::Color32::RED);

                frame.show(ui, |ui| {
                    ui.heading("teste");
                });
            });
        });
    }
}
