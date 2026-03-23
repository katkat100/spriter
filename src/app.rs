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
}

impl eframe::App for Spriter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.playback.playing {
            ctx.request_repaint();
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
