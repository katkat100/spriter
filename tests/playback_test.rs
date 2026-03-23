use spriter::model::playback::PlaybackState;

#[test]
fn test_advance_frame_wraps_when_looping() {
    let mut state = PlaybackState::new();
    state.playing = true;
    let total_frames = 4;

    // Simulate enough elapsed time for one frame at 10 FPS (0.1s per frame)
    state.advance(0.1, 10.0, total_frames, true);
    assert_eq!(state.current_frame_index, 1);

    // Advance to end and wrap
    state.current_frame_index = 3;
    state.elapsed = 0.0;
    state.advance(0.1, 10.0, total_frames, true);
    assert_eq!(state.current_frame_index, 0);
}

#[test]
fn test_advance_frame_stops_when_not_looping() {
    let mut state = PlaybackState::new();
    state.playing = true;
    state.current_frame_index = 3;
    let total_frames = 4;

    state.advance(0.1, 10.0, total_frames, false);
    assert_eq!(state.current_frame_index, 3);
    assert!(!state.playing);
}

#[test]
fn test_advance_does_nothing_when_paused() {
    let mut state = PlaybackState::new();
    state.playing = false;
    state.advance(1.0, 10.0, 4, true);
    assert_eq!(state.current_frame_index, 0);
}

#[test]
fn test_step_forward_and_back() {
    let mut state = PlaybackState::new();
    let total_frames = 4;

    state.step_forward(total_frames);
    assert_eq!(state.current_frame_index, 1);

    state.step_forward(total_frames);
    assert_eq!(state.current_frame_index, 2);

    state.step_back();
    assert_eq!(state.current_frame_index, 1);

    state.step_back();
    assert_eq!(state.current_frame_index, 0);

    // step_back at 0 stays at 0
    state.step_back();
    assert_eq!(state.current_frame_index, 0);
}

#[test]
fn test_jump_to_first_and_last() {
    let mut state = PlaybackState::new();
    state.current_frame_index = 2;

    state.jump_to_first();
    assert_eq!(state.current_frame_index, 0);

    state.jump_to_last(8);
    assert_eq!(state.current_frame_index, 7);
}
