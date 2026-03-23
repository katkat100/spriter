use spriter::io::project_file::{load_project, save_project};
use spriter::io::sprite_sheet::SpriteSheet;
use spriter::model::playback::PlaybackState;
use spriter::model::project::{Animation, Preset, Project};
use std::path::PathBuf;

#[test]
fn test_full_project_workflow() {
    // Create a project
    let mut project = Project::default();
    project.name = "integration-test".to_string();
    project.sprite_sheet = PathBuf::from("test-sheet.png");
    project.frame_width = 32;
    project.frame_height = 32;
    project.presets.push(Preset {
        name: "default".to_string(),
        animations: vec![
            Animation {
                name: "walk".to_string(),
                frames: vec![0, 1, 2, 3],
                fps: 10.0,
                looping: true,
            },
            Animation {
                name: "idle".to_string(),
                frames: vec![4, 5],
                fps: 6.0,
                looping: true,
            },
        ],
    });

    // Save and reload
    let dir = std::env::temp_dir().join("spriter_integration_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.spriter.toml");
    save_project(&project, &path).unwrap();
    let loaded = load_project(&path).unwrap();

    assert_eq!(loaded.name, "integration-test");
    assert_eq!(loaded.presets.len(), 1);
    assert_eq!(loaded.presets[0].animations.len(), 2);

    // Test playback on the walk animation
    let walk = &loaded.presets[0].animations[0];
    let mut playback = PlaybackState::new();
    playback.playing = true;

    // At 10 FPS, frame duration = 0.1s. Use 0.11s to avoid floating-point boundary issues.
    for expected_frame in 1..4 {
        playback.advance(0.11, walk.fps, walk.frames.len(), walk.looping);
        assert_eq!(playback.current_frame_index, expected_frame);
    }

    // Should wrap to 0
    playback.advance(0.11, walk.fps, walk.frames.len(), walk.looping);
    assert_eq!(playback.current_frame_index, 0);

    // Test grid math
    let sheet = SpriteSheet::new(128, 64, 32, 32);
    assert_eq!(sheet.total_frames(), 8);
    assert_eq!(sheet.frame_origin(5), (32, 32));

    std::fs::remove_dir_all(&dir).ok();
}
