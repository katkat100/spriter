use egui::Ui;

use crate::app::Spriter;

pub fn show(ui: &mut Ui, app: &mut Spriter) {
    let total_frames = app.current_animation()
        .map(|a| a.frames.len())
        .unwrap_or(0);

    ui.horizontal(|ui| {
        if ui.button("⏮").on_hover_text("First frame").clicked() {
            app.playback.jump_to_first();
        }
        if ui.button("◀").on_hover_text("Previous frame").clicked() {
            app.playback.step_back();
        }

        let play_label = if app.playback.playing { "⏸" } else { "▶" };
        if ui.button(play_label).on_hover_text("Play/Pause").clicked() {
            app.playback.toggle_play();
        }

        if ui.button("▶▶").on_hover_text("Next frame").clicked() {
            app.playback.step_forward(total_frames);
        }
        if ui.button("⏭").on_hover_text("Last frame").clicked() {
            app.playback.jump_to_last(total_frames);
        }

        ui.separator();

        let mut fps = app.current_animation().map(|a| a.fps).unwrap_or(12.0);
        ui.label("FPS:");
        if ui.add(egui::Slider::new(&mut fps, 1.0..=60.0).fixed_decimals(0)).changed() {
            if let Some(preset) = app.project.presets.get_mut(app.selected_preset) {
                if let Some(anim) = preset.animations.get_mut(app.selected_animation) {
                    anim.fps = fps;
                }
            }
        }

        ui.separator();

        let mut looping = app.current_animation().map(|a| a.looping).unwrap_or(true);
        if ui.checkbox(&mut looping, "Loop").changed() {
            if let Some(preset) = app.project.presets.get_mut(app.selected_preset) {
                if let Some(anim) = preset.animations.get_mut(app.selected_animation) {
                    anim.looping = looping;
                }
            }
        }
    });
}
