pub mod pages;
pub mod controllers;

use eframe::egui;
use crate::controllers::page_controller::PageController;

fn main() {
    let opcoes_nativas = eframe::NativeOptions::default();
    eframe::run_native("Deskho", opcoes_nativas, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

struct MyEguiApp {
    page_controller: PageController,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let page_controller = PageController::new();
        Self{page_controller}
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           self.page_controller.draw_page(ui);
       });
   }
}
