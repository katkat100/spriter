pub struct PlaybackState {
    pub current_frame_index: usize,
    pub playing: bool,
    pub elapsed: f32,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            current_frame_index: 0,
            playing: false,
            elapsed: 0.0,
        }
    }
}
