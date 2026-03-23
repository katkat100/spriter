mod app;
mod io;
mod model;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("Spriter"),
        ..Default::default()
    };
    eframe::run_native("Spriter", options, Box::new(|cc| Ok(Box::new(app::Spriter::new(cc)))))
}
