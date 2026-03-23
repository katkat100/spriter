use eframe::egui;

use crate::io::sprite_sheet::SpriteSheet;
use crate::model::playback::PlaybackState;
use crate::model::project::Project;

pub struct Spriter {
    pub project: Project,
    pub playback: PlaybackState,
    pub selected_preset: usize,
    pub selected_animation: usize,
    pub sheet: Option<SpriteSheet>,
    pub frame_textures: Vec<egui::TextureHandle>,
    pub error_message: Option<String>,
    pub pending_sheet_load: Option<std::path::PathBuf>,
    pub show_frame_size_dialog: bool,
    pub frame_size_input: [String; 2],
}

impl Spriter {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            project: Project::default(),
            playback: PlaybackState::new(),
            selected_preset: 0,
            selected_animation: 0,
            sheet: None,
            frame_textures: Vec::new(),
            error_message: None,
            pending_sheet_load: None,
            show_frame_size_dialog: false,
            frame_size_input: ["32".to_string(), "32".to_string()],
        }
    }

    pub fn current_animation(&self) -> Option<&crate::model::project::Animation> {
        let preset = self.project.presets.get(self.selected_preset)?;
        preset.animations.get(self.selected_animation)
    }

    pub fn load_sprite_sheet(&mut self, path: &std::path::Path, ctx: &egui::Context) {
        match SpriteSheet::load_frames(path, self.project.frame_width, self.project.frame_height) {
            Ok((sheet, frames)) => {
                self.frame_textures = frames.into_iter().enumerate().map(|(i, img)| {
                    ctx.load_texture(
                        format!("frame_{i}"),
                        img,
                        egui::TextureOptions::NEAREST,
                    )
                }).collect();
                if sheet.remainder_x() > 0 || sheet.remainder_y() > 0 {
                    self.error_message = Some(format!(
                        "Warning: image not evenly divisible — {}px clipped horizontally, {}px clipped vertically",
                        sheet.remainder_x(), sheet.remainder_y()
                    ));
                }
                self.sheet = Some(sheet);
            }
            Err(e) => {
                self.error_message = Some(e);
            }
        }
    }
}

impl eframe::App for Spriter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Advance playback
        if self.playback.playing {
            ctx.request_repaint();
            if let Some(anim) = self.current_animation().cloned() {
                let dt = ctx.input(|i| i.stable_dt);
                self.playback.advance(dt, anim.fps, anim.frames.len(), anim.looping);
            }
        }

        // Handle dropped files
        let dropped: Vec<_> = ctx.input(|i| i.raw.dropped_files.clone());
        for file in &dropped {
            if let Some(path) = &file.path {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                match ext {
                    "toml" => {
                        match crate::io::project_file::load_project(path) {
                            Ok(project) => {
                                self.project = project;
                                self.selected_preset = 0;
                                self.selected_animation = 0;
                                self.error_message = None;
                                let sheet_path = path.parent()
                                    .unwrap_or(std::path::Path::new("."))
                                    .join(&self.project.sprite_sheet);
                                self.load_sprite_sheet(&sheet_path, ctx);
                            }
                            Err(e) => self.error_message = Some(e),
                        }
                    }
                    "png" | "jpg" | "jpeg" | "bmp" => {
                        self.project.sprite_sheet = path.clone();
                        self.show_frame_size_dialog = true;
                        self.frame_size_input = [
                            self.project.frame_width.to_string(),
                            self.project.frame_height.to_string(),
                        ];
                    }
                    _ => {
                        self.error_message = Some(format!("Unsupported file type: .{ext}"));
                    }
                }
            }
        }

        // Handle deferred sprite sheet load from file dialog or CLI
        if let Some(path) = self.pending_sheet_load.take() {
            if path.exists() {
                self.load_sprite_sheet(&path, ctx);
            } else {
                self.error_message = Some(format!(
                    "Sprite sheet not found: {}. Use Open to locate it.",
                    path.display()
                ));
            }
        }

        egui::SidePanel::left("side_panel")
            .default_width(220.0)
            .show(ctx, |ui| {
                crate::ui::side_panel::show(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::canvas::show(ui, self);
            ui.separator();
            crate::ui::playback_bar::show(ui, self);
        });
    }
}
