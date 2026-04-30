use egui::{Color32, Rect, Sense, Ui, Vec2};

use crate::app::Spriter;
use crate::io::project_file;
use crate::model::project::{Animation, Preset};
use crate::tab::Tab;
use super::widgets::{retro_button, retro_heading, retro_selectable, ACCENT};

pub fn show(ui: &mut Ui, app: &mut Spriter) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        show_inner(ui, app);
    });
}

fn show_inner(ui: &mut Ui, app: &mut Spriter) {
    ui.add_space(4.0);
    retro_heading(ui, "SPRITER");
    ui.add_space(4.0);

    // Project name
    retro_heading(ui, "PROJECT");
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut app.tabs[app.active_tab].project.name);
    });

    ui.horizontal(|ui| {
        if retro_button(ui, "NEW").clicked() {
            app.add_tab(Tab::new());
        }
        if retro_button(ui, "OPEN").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Spriter Project", &["toml"])
                .add_filter("Image", &["png", "bmp", "jpg", "jpeg"])
                .pick_file()
            {
                let is_toml = path.extension().is_some_and(|ext| ext == "toml");
                if is_toml {
                    match project_file::load_project(&path) {
                        Ok(project) => {
                            let mut tab = Tab::new();
                            let sheet_path = path.parent()
                                .unwrap_or(std::path::Path::new("."))
                                .join(&project.sprite_sheet);
                            tab.project = project;
                            tab.save_path = Some(path.clone());
                            tab.pending_sheet_load = Some(sheet_path);
                            app.add_tab(tab);
                            app.config.add_recent(path);
                            app.save_session();
                        }
                        Err(e) => app.error_message = Some(e),
                    }
                } else {
                    app.open_image_file(&path);
                }
            }
        }
        if retro_button(ui, "SAVE").clicked() {
            let existing_path = app.tabs[app.active_tab].save_path.clone();
            let path = if let Some(p) = existing_path {
                Some(p)
            } else {
                let tab = &app.tabs[app.active_tab];
                let default_name = format!("{}.spriter.toml", tab.project.name);
                rfd::FileDialog::new()
                    .add_filter("Spriter Project", &["toml"])
                    .set_file_name(&default_name)
                    .save_file()
            };
            if let Some(path) = path {
                let tab = &mut app.tabs[app.active_tab];
                match project_file::save_project(&tab.project, &path) {
                    Ok(()) => {
                        tab.save_path = Some(path.clone());
                        app.config.add_recent(path);
                        app.save_session();
                        app.status_message = Some("Project saved".to_string());
                    }
                    Err(e) => app.error_message = Some(e),
                }
            }
        }
    });

    // Recent files
    if !app.config.recent_files.is_empty() {
        ui.collapsing("Recent", |ui| {
            let recents = app.config.recent_files.clone();
            for path in &recents {
                let label = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                if retro_selectable(ui, label, false).on_hover_text(path.display().to_string()).clicked() {
                    if path.exists() {
                        match project_file::load_project(path) {
                            Ok(project) => {
                                let mut tab = Tab::new();
                                let sheet_path = path.parent()
                                    .unwrap_or(std::path::Path::new("."))
                                    .join(&project.sprite_sheet);
                                tab.project = project;
                                tab.save_path = Some(path.clone());
                                tab.pending_sheet_load = Some(sheet_path);
                                app.add_tab(tab);
                                app.config.add_recent(path.clone());
                                app.save_session();
                            }
                            Err(e) => app.error_message = Some(e),
                        }
                    } else {
                        app.error_message = Some(format!("File not found: {}", path.display()));
                    }
                }
            }
        });
    }

    // Sprite sheet preview
    let sheet_tex_id = app.tabs[app.active_tab].sheet_texture.as_ref().map(|t| (t.id(), t.size_vec2()));
    if let Some((tex_id, tex_size)) = sheet_tex_id {
        ui.add_space(4.0);
        let panel_width = ui.available_width();
        let scale = panel_width / tex_size.x;
        let display_size = Vec2::new(panel_width, tex_size.y * scale);
        ui.image(egui::load::SizedTexture::new(tex_id, display_size));

        // Refresh button
        if retro_button(ui, "REFRESH").clicked() {
            let tab = &mut app.tabs[app.active_tab];
            match tab.refresh_sprite_sheet(ui.ctx()) {
                Ok(()) => app.status_message = Some("Sprite sheet refreshed".to_string()),
                Err(e) => app.error_message = Some(e),
            }
        }
    }

    ui.add_space(6.0);

    // Preset section
    retro_heading(ui, "PRESETS");
    let tab = &app.tabs[app.active_tab];
    let preset_names: Vec<String> = tab.project.presets.iter().map(|p| p.name.clone()).collect();
    let _ = tab;
    for (i, name) in preset_names.iter().enumerate() {
        let selected = i == app.tabs[app.active_tab].selected_preset;
        if retro_selectable(ui, name, selected).clicked() {
            let tab = &mut app.tabs[app.active_tab];
            tab.selected_preset = i;
            tab.selected_animation = 0;
            tab.playback.jump_to_first();
        }
    }
    if retro_button(ui, "+ PRESET").clicked() {
        let count = app.tabs[app.active_tab].project.presets.len();
        app.tabs[app.active_tab].project.presets.push(Preset {
            name: format!("preset-{}", count),
            animations: Vec::new(),
        });
    }

    ui.add_space(6.0);

    // Animation section
    retro_heading(ui, "ANIMATIONS");
    let tab = &app.tabs[app.active_tab];
    let anim_names: Vec<String> = tab.project.presets.get(tab.selected_preset)
        .map(|preset| preset.animations.iter()
            .map(|a| format!("{} ({}-{})",
                a.name,
                a.frames.first().unwrap_or(&0),
                a.frames.last().unwrap_or(&0)))
            .collect())
        .unwrap_or_default();
    let _ = tab;
    for (i, name) in anim_names.iter().enumerate() {
        let selected = i == app.tabs[app.active_tab].selected_animation;
        if retro_selectable(ui, name, selected).clicked() {
            let tab = &mut app.tabs[app.active_tab];
            tab.selected_animation = i;
            tab.playback.jump_to_first();
        }
    }
    if retro_button(ui, "+ ANIMATION").clicked() {
        let tab = &mut app.tabs[app.active_tab];
        if let Some(preset) = tab.project.presets.get_mut(tab.selected_preset) {
            let new_index = preset.animations.len();
            preset.animations.push(Animation {
                name: format!("anim-{}", new_index),
                frames: Vec::new(),
                fps: 12.0,
                looping: true,
                ping_pong: false,
            });
            tab.selected_animation = new_index;
            tab.playback.jump_to_first();
        }
    }

    // Animation editor for selected animation
    let tab = &mut app.tabs[app.active_tab];
    let sel_preset = tab.selected_preset;
    let sel_anim = tab.selected_animation;
    if let Some(preset) = tab.project.presets.get_mut(sel_preset)
        && let Some(anim) = preset.animations.get_mut(sel_anim)
    {
        ui.add_space(6.0);
        retro_heading(ui, "EDIT ANIMATION");
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut anim.name);
        });
        ui.horizontal(|ui| {
            ui.label("FPS:");
            ui.add(egui::Slider::new(&mut anim.fps, 1.0..=60.0));
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut anim.looping, "Loop");
            ui.checkbox(&mut anim.ping_pong, "Ping-pong");
        });

        // Sync the text buffer when switching to a different animation
        let current_anim_id = (sel_preset, sel_anim);
        if tab.frames_input_anim_id != Some(current_anim_id) {
            tab.frames_input = anim.frames.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            tab.frames_input_anim_id = Some(current_anim_id);
        }

        ui.horizontal(|ui| {
            let total = tab.frame_textures.len();
            if total > 0 {
                ui.label(format!("Frames (0-{}):", total - 1));
            } else {
                ui.label("Frames:");
            }
            let response = ui.text_edit_singleline(&mut tab.frames_input);
            if response.lost_focus() {
                anim.frames = tab.frames_input.split(',')
                    .filter_map(|s| s.trim().parse::<usize>().ok())
                    .collect();
                tab.frames_input = anim.frames.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
            }
        });

        // Frame grid selector
        if !tab.frame_textures.is_empty() {
            ui.add_space(4.0);
            ui.label("Click to toggle frames:");
            let cell_size = 40.0;
            let spacing = 2.0;
            let panel_width = ui.available_width();
            let cols = ((panel_width + spacing) / (cell_size + spacing)).floor().max(1.0) as usize;

            let total = tab.frame_textures.len();
            let rows = (total + cols - 1) / cols;
            let mut toggled: Option<usize> = None;

            for row in 0..rows {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                    for col in 0..cols {
                        let idx = row * cols + col;
                        if idx >= total { break; }
                        let is_selected = anim.frames.contains(&idx);
                        let texture = &tab.frame_textures[idx];

                        let (response, painter) = ui.allocate_painter(
                            Vec2::splat(cell_size),
                            Sense::click(),
                        );
                        let rect = response.rect;

                        // Background with pixel-art border
                        let bg = if is_selected {
                            ACCENT.gamma_multiply(0.35)
                        } else if response.hovered() {
                            Color32::from_rgb(55, 55, 68)
                        } else {
                            Color32::from_rgb(35, 35, 42)
                        };
                        painter.rect_filled(rect, 0.0, bg);

                        // Frame thumbnail
                        let inset = rect.shrink(2.0);
                        painter.image(
                            texture.id(),
                            inset,
                            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );

                        // Border
                        if is_selected {
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, ACCENT), egui::StrokeKind::Outside);
                        } else {
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, Color32::from_rgb(55, 55, 65)), egui::StrokeKind::Outside);
                        }

                        // Index label
                        painter.text(
                            rect.left_top() + Vec2::new(2.0, 1.0),
                            egui::Align2::LEFT_TOP,
                            idx.to_string(),
                            egui::FontId::new(13.0, egui::FontFamily::Monospace),
                            if is_selected { Color32::WHITE } else { Color32::from_gray(160) },
                        );

                        if response.clicked() {
                            toggled = Some(idx);
                        }
                    }
                });
            }

            if let Some(idx) = toggled {
                if let Some(pos) = anim.frames.iter().position(|&f| f == idx) {
                    anim.frames.remove(pos);
                } else {
                    anim.frames.push(idx);
                }
                tab.frames_input = anim.frames.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
            }
        }
    }

    // Status display
    if let Some(msg) = &app.status_message {
        ui.add_space(6.0);
        ui.colored_label(Color32::from_rgb(80, 200, 120), msg.as_str());
        if retro_button(ui, "OK").clicked() {
            app.status_message = None;
        }
    }

    // Error display
    if let Some(err) = &app.error_message {
        ui.add_space(6.0);
        ui.colored_label(Color32::from_rgb(220, 60, 60), format!("! {err}"));
        if retro_button(ui, "OK").clicked() {
            app.error_message = None;
        }
    }
}
