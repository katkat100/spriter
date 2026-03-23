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
}
