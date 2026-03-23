use clap::Parser;
use std::path::PathBuf;

use spriter::app;
use spriter::io;

#[derive(Parser)]
#[command(name = "spriter", about = "Sprite animation viewer for game development")]
struct Cli {
    /// Path to a sprite sheet image (.png, .jpg, .bmp) or project file (.spriter.toml)
    file: Option<PathBuf>,

    /// Frame width in pixels (required when loading an image directly)
    #[arg(short = 'W', long)]
    frame_width: Option<u32>,

    /// Frame height in pixels (required when loading an image directly)
    #[arg(short = 'H', long)]
    frame_height: Option<u32>,
}

fn main() -> eframe::Result<()> {
    let cli = Cli::parse();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("Spriter")
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native("Spriter", options, Box::new(move |cc| {
        let mut app = app::Spriter::new(cc);

        if let Some(path) = &cli.file {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            match ext {
                "toml" => {
                    match io::project_file::load_project(path) {
                        Ok(project) => {
                            app.project = project;
                            let sheet_path = path.parent()
                                .unwrap_or(std::path::Path::new("."))
                                .join(&app.project.sprite_sheet);
                            app.pending_sheet_load = Some(sheet_path);
                        }
                        Err(e) => app.error_message = Some(e),
                    }
                }
                "png" | "jpg" | "jpeg" | "bmp" => {
                    if let (Some(w), Some(h)) = (cli.frame_width, cli.frame_height) {
                        app.project.frame_width = w;
                        app.project.frame_height = h;
                    }
                    app.project.sprite_sheet = path.clone();
                    app.pending_sheet_load = Some(path.clone());
                }
                _ => {
                    app.error_message = Some(format!("Unsupported file type: .{ext}"));
                }
            }
        }

        Ok(Box::new(app))
    }))
}
