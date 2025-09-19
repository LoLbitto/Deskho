pub mod main_page;
pub mod main_states;

use egui::Ui;

pub trait Page {
    fn update(&mut self, ui: &mut Ui);
}
