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

    pub fn advance(&mut self, dt: f32, fps: f32, total_frames: usize, looping: bool) {
        if !self.playing || total_frames == 0 {
            return;
        }

        self.elapsed += dt;
        let frame_duration = 1.0 / fps;

        while self.elapsed >= frame_duration {
            self.elapsed -= frame_duration;
            let next = self.current_frame_index + 1;
            if next >= total_frames {
                if looping {
                    self.current_frame_index = 0;
                } else {
                    self.playing = false;
                    return;
                }
            } else {
                self.current_frame_index = next;
            }
        }
    }

    pub fn step_forward(&mut self, total_frames: usize) {
        if self.current_frame_index + 1 < total_frames {
            self.current_frame_index += 1;
        }
        self.elapsed = 0.0;
    }

    pub fn step_back(&mut self) {
        self.current_frame_index = self.current_frame_index.saturating_sub(1);
        self.elapsed = 0.0;
    }

    pub fn jump_to_first(&mut self) {
        self.current_frame_index = 0;
        self.elapsed = 0.0;
    }

    pub fn jump_to_last(&mut self, total_frames: usize) {
        if total_frames > 0 {
            self.current_frame_index = total_frames - 1;
        }
        self.elapsed = 0.0;
    }

    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
        if self.playing {
            self.elapsed = 0.0;
        }
    }
}
