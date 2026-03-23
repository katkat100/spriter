use egui::Ui;

use crate::app::Spriter;
use crate::io::project_file;
use crate::model::playback::PlaybackState;
use crate::model::project::{Animation, Preset, Project};

pub fn show(ui: &mut Ui, app: &mut Spriter) {
    ui.heading("Spriter");
    ui.separator();

    // Project section
    ui.label("Project");
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.project.name);
    });

    ui.horizontal(|ui| {
        if ui.button("New").clicked() {
            app.project = Project::default();
            app.playback = PlaybackState::new();
            app.selected_preset = 0;
            app.selected_animation = 0;
            app.sheet = None;
            app.frame_textures.clear();
            app.pending_sheet_load = None;
            app.show_frame_size_dialog = false;
            app.error_message = None;
        }
        if ui.button("Open").clicked()
            && let Some(path) = rfd::FileDialog::new()
                .add_filter("Spriter Project", &["toml"])
                .add_filter("Image", &["png", "bmp", "jpg", "jpeg"])
                .pick_file()
        {
            if path.extension().is_some_and(|ext| ext == "toml") {
                match project_file::load_project(&path) {
                    Ok(project) => {
                        app.project = project;
                        app.selected_preset = 0;
                        app.selected_animation = 0;
                        app.error_message = None;
                        let sheet_path = path.parent()
                            .unwrap_or(std::path::Path::new("."))
                            .join(&app.project.sprite_sheet);
                        app.pending_sheet_load = Some(sheet_path);
                    }
                    Err(e) => app.error_message = Some(e),
                }
            } else {
                app.project.sprite_sheet = path;
                app.show_frame_size_dialog = true;
                app.frame_size_input = [
                    app.project.frame_width.to_string(),
                    app.project.frame_height.to_string(),
                ];
            }
        }
        if ui.button("Save").clicked()
            && let Some(path) = rfd::FileDialog::new()
                .add_filter("Spriter Project", &["toml"])
                .set_file_name(format!("{}.spriter.toml", app.project.name))
                .save_file()
            && let Err(e) = project_file::save_project(&app.project, &path)
        {
            app.error_message = Some(e);
        }
    });

    ui.separator();

    // Preset section
    ui.label("Presets");
    let preset_names: Vec<String> = app.project.presets.iter().map(|p| p.name.clone()).collect();
    for (i, name) in preset_names.iter().enumerate() {
        let selected = i == app.selected_preset;
        if ui.selectable_label(selected, name).clicked() {
            app.selected_preset = i;
            app.selected_animation = 0;
            app.playback.jump_to_first();
        }
    }
    if ui.small_button("+ Add preset").clicked() {
        app.project.presets.push(Preset {
            name: format!("preset-{}", app.project.presets.len()),
            animations: Vec::new(),
        });
    }

    ui.separator();

    // Animation section
    ui.label("Animations");
    if let Some(preset) = app.project.presets.get(app.selected_preset) {
        let anim_names: Vec<String> = preset.animations.iter()
            .map(|a| format!("{} ({}-{})",
                a.name,
                a.frames.first().unwrap_or(&0),
                a.frames.last().unwrap_or(&0)))
            .collect();
        for (i, name) in anim_names.iter().enumerate() {
            let selected = i == app.selected_animation;
            if ui.selectable_label(selected, name).clicked() {
                app.selected_animation = i;
                app.playback.jump_to_first();
            }
        }
    }
    if ui.small_button("+ Add animation").clicked()
        && let Some(preset) = app.project.presets.get_mut(app.selected_preset)
    {
        preset.animations.push(Animation {
            name: format!("anim-{}", preset.animations.len()),
            frames: Vec::new(),
            fps: 12.0,
            looping: true,
        });
    }

    // Animation editor for selected animation
    if let Some(preset) = app.project.presets.get_mut(app.selected_preset)
        && let Some(anim) = preset.animations.get_mut(app.selected_animation)
    {
        ui.separator();
        ui.label("Edit Animation");
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut anim.name);
        });
        ui.horizontal(|ui| {
            ui.label("FPS:");
            ui.add(egui::Slider::new(&mut anim.fps, 1.0..=60.0));
        });
        ui.checkbox(&mut anim.looping, "Loop");

        ui.horizontal(|ui| {
            ui.label("Frames:");
            let mut frame_str = anim.frames.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if ui.text_edit_singleline(&mut frame_str).changed() {
                anim.frames = frame_str.split(',')
                    .filter_map(|s| s.trim().parse::<usize>().ok())
                    .collect();
            }
        });
    }

    // Error display
    if let Some(err) = &app.error_message {
        ui.separator();
        ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
        if ui.small_button("Dismiss").clicked() {
            app.error_message = None;
        }
    }
}
