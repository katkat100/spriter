use egui::{Color32, Rect, Sense, Ui, Vec2};

use crate::app::Spriter;

pub fn show(ui: &mut Ui, app: &Spriter) {
    let available = ui.available_size();
    // Reserve space for playback bar
    let canvas_height = (available.y - 50.0).max(100.0);
    let (response, painter) = ui.allocate_painter(
        Vec2::new(available.x, canvas_height),
        Sense::hover(),
    );
    let rect = response.rect;

    // Draw checkerboard background
    draw_checkerboard(&painter, rect, 16.0);

    // Draw current frame if textures are loaded
    if !app.frame_textures.is_empty()
        && let Some(anim) = app.current_animation()
        && !anim.frames.is_empty()
    {
        let frame_idx = anim.frames[app.playback.current_frame_index % anim.frames.len()];
        if let Some(texture) = app.frame_textures.get(frame_idx) {
            let tex_size = texture.size_vec2();
            // Scale to fit canvas while maintaining aspect ratio
            let scale = (rect.width() / tex_size.x)
                .min(rect.height() / tex_size.y)
                .min(4.0); // max 4x zoom
            let display_size = tex_size * scale;
            let center = rect.center();
            let img_rect = Rect::from_center_size(center, display_size);

            ui.painter().image(
                texture.id(),
                img_rect,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    } else {
        // No image loaded — show hint
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Drop a sprite sheet here\nor use Open to load one",
            egui::FontId::proportional(16.0),
            Color32::from_gray(120),
        );
    }

    // Frame counter
    if let Some(anim) = app.current_animation()
        && !anim.frames.is_empty()
    {
        let total = anim.frames.len();
        let current = app.playback.current_frame_index % total;
        let text = format!("Frame {}/{}", current + 1, total);
        ui.painter().text(
            egui::pos2(rect.right() - 10.0, rect.bottom() - 10.0),
            egui::Align2::RIGHT_BOTTOM,
            text,
            egui::FontId::monospace(13.0),
            Color32::from_gray(180),
        );
    }
}

fn draw_checkerboard(painter: &egui::Painter, rect: Rect, cell_size: f32) {
    let light = Color32::from_gray(50);
    let dark = Color32::from_gray(35);

    painter.rect_filled(rect, 0.0, dark);

    let cols = (rect.width() / cell_size).ceil() as usize;
    let rows = (rect.height() / cell_size).ceil() as usize;

    for row in 0..rows {
        for col in 0..cols {
            if (row + col) % 2 == 0 {
                let x = rect.left() + col as f32 * cell_size;
                let y = rect.top() + row as f32 * cell_size;
                let cell_rect = Rect::from_min_size(
                    egui::pos2(x, y),
                    Vec2::new(cell_size, cell_size),
                ).intersect(rect);
                painter.rect_filled(cell_rect, 0.0, light);
            }
        }
    }
}
