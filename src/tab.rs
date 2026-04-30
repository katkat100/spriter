use eframe::egui;

use crate::io::sprite_sheet::SpriteSheet;
use crate::model::playback::PlaybackState;
use crate::model::project::Project;

pub struct Tab {
    pub project: Project,
    pub playback: PlaybackState,
    pub selected_preset: usize,
    pub selected_animation: usize,
    pub sheet: Option<SpriteSheet>,
    pub sheet_texture: Option<egui::TextureHandle>,
    pub frame_textures: Vec<egui::TextureHandle>,
    pub pending_sheet_load: Option<std::path::PathBuf>,
    pub frames_input: String,
    pub frames_input_anim_id: Option<(usize, usize)>,
    pub save_path: Option<std::path::PathBuf>,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            project: Project::default(),
            playback: PlaybackState::new(),
            selected_preset: 0,
            selected_animation: 0,
            sheet: None,
            sheet_texture: None,
            frame_textures: Vec::new(),
            pending_sheet_load: None,
            frames_input: String::new(),
            frames_input_anim_id: None,
            save_path: None,
        }
    }

    pub fn current_animation(&self) -> Option<&crate::model::project::Animation> {
        let preset = self.project.presets.get(self.selected_preset)?;
        preset.animations.get(self.selected_animation)
    }

    pub fn load_sprite_sheet(&mut self, path: &std::path::Path, ctx: &egui::Context) -> Result<(), String> {
        match SpriteSheet::load_frames(path, self.project.frame_width, self.project.frame_height) {
            Ok((sheet, full_image, frames)) => {
                self.sheet_texture = Some(ctx.load_texture(
                    "sprite_sheet",
                    full_image,
                    egui::TextureOptions::NEAREST,
                ));
                self.frame_textures = frames.into_iter().enumerate().map(|(i, img)| {
                    ctx.load_texture(
                        format!("frame_{i}"),
                        img,
                        egui::TextureOptions::NEAREST,
                    )
                }).collect();
                // Auto-create a default preset + animation so frames are visible immediately
                let frame_count = self.frame_textures.len();
                if self.project.presets.is_empty() && frame_count > 0 {
                    self.project.presets.push(crate::model::project::Preset {
                        name: "default".to_string(),
                        animations: vec![crate::model::project::Animation {
                            name: "all-frames".to_string(),
                            frames: (0..frame_count).collect(),
                            fps: 12.0,
                            looping: true,
                            ping_pong: false,
                        }],
                    });
                    self.selected_preset = 0;
                    self.selected_animation = 0;
                    self.playback.jump_to_first();
                }
                let remainder_msg = if sheet.remainder_x() > 0 || sheet.remainder_y() > 0 {
                    Some(format!(
                        "Image not evenly divisible — {}px clipped horizontally, {}px clipped vertically",
                        sheet.remainder_x(), sheet.remainder_y()
                    ))
                } else {
                    None
                };
                self.sheet = Some(sheet);
                if let Some(msg) = remainder_msg {
                    return Err(msg); // Caller can treat as status, not fatal error
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn refresh_sprite_sheet(&mut self, ctx: &egui::Context) -> Result<(), String> {
        // Return early if no sprite sheet loaded
        let sheet = match &self.sheet {
            Some(s) => s,
            None => return Ok(()),
        };

        // Get current frame dimensions
        let frame_width = sheet.frame_width;
        let frame_height = sheet.frame_height;

        // Get the sprite sheet path
        let path = &self.project.sprite_sheet;
        if path.as_os_str().is_empty() {
            return Ok(());
        }

        // Reload the sprite sheet
        match SpriteSheet::load_frames(path, frame_width, frame_height) {
            Ok((new_sheet, full_image, frames)) => {
                self.sheet_texture = Some(ctx.load_texture(
                    "sprite_sheet",
                    full_image,
                    egui::TextureOptions::NEAREST,
                ));
                self.frame_textures = frames.into_iter().enumerate().map(|(i, img)| {
                    ctx.load_texture(
                        format!("frame_{i}"),
                        img,
                        egui::TextureOptions::NEAREST,
                    )
                }).collect();
                self.sheet = Some(new_sheet);
                Ok(())
            }
            Err(e) => Err(format!("Failed to refresh: {e}")),
        }
    }

    pub fn display_name(&self) -> &str {
        let name = &self.project.name;
        if name.is_empty() || name == "untitled" {
            "untitled"
        } else {
            name
        }
    }
}
