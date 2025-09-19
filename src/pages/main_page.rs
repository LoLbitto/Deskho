use crate::pages::Page;
use egui::Ui;

pub struct MainPage {}

impl Page for MainPage {
    fn update(&mut self, ui: &mut Ui) {
        ui.heading("teste pagina");
    }
}
