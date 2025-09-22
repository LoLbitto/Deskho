use crate::pages::Page;
use crate::pages::main_page::MainPage;
use egui::Ui;

pub struct PageController {
    page: Box<dyn Page>,
    // ui: # vai ser uns elementos q persistem entre páginas 
}

impl PageController {
    pub fn new() -> Self {
        let page = Box::new(MainPage{});
        Self {page}
    }

    pub fn change_page(&mut self, page: Box<dyn Page>) {
        self.page = page;
    }

    pub fn draw_page(&mut self, ui: &mut Ui) {
        self.page.update(ui);
    }
}
