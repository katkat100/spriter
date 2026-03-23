use egui;
use image::GenericImageView;

pub struct SpriteSheet {
    pub width: u32,
    pub height: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub columns: u32,
    pub rows: u32,
}

impl SpriteSheet {
    pub fn new(image_width: u32, image_height: u32, frame_width: u32, frame_height: u32) -> Self {
        Self {
            width: image_width,
            height: image_height,
            frame_width,
            frame_height,
            columns: image_width / frame_width,
            rows: image_height / frame_height,
        }
    }

    pub fn total_frames(&self) -> usize {
        (self.columns * self.rows) as usize
    }

    pub fn frame_origin(&self, frame_index: usize) -> (u32, u32) {
        let col = (frame_index as u32) % self.columns;
        let row = (frame_index as u32) / self.columns;
        (col * self.frame_width, row * self.frame_height)
    }

    pub fn remainder_x(&self) -> u32 {
        self.width % self.frame_width
    }

    pub fn remainder_y(&self) -> u32 {
        self.height % self.frame_height
    }

    pub fn load_frames(path: &std::path::Path, frame_width: u32, frame_height: u32)
        -> Result<(Self, Vec<egui::ColorImage>), String>
    {
        let img = image::open(path).map_err(|e| format!("Failed to load image: {e}"))?;
        let (img_w, img_h) = img.dimensions();
        let sheet = Self::new(img_w, img_h, frame_width, frame_height);
        let rgba = img.to_rgba8();

        let mut frames = Vec::with_capacity(sheet.total_frames());
        for idx in 0..sheet.total_frames() {
            let (ox, oy) = sheet.frame_origin(idx);
            let mut pixels = Vec::with_capacity((frame_width * frame_height) as usize);
            for y in oy..(oy + frame_height) {
                for x in ox..(ox + frame_width) {
                    let p = rgba.get_pixel(x, y);
                    pixels.push(egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]));
                }
            }
            frames.push(egui::ColorImage {
                size: [frame_width as usize, frame_height as usize],
                pixels,
            });
        }

        Ok((sheet, frames))
    }
}
