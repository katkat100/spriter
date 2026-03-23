use eframe::egui;

pub struct Spriter {
    // State will be added in later tasks
}

impl Spriter {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for Spriter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Spriter");
            ui.label("Sprite animation viewer");
        });
    }
}
