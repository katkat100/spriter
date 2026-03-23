use spriter::io::project_file::{load_project, save_project};
use spriter::model::project::{Animation, Preset, Project};
use std::path::PathBuf;

#[test]
fn test_save_and_load_round_trip() {
    let dir = std::env::temp_dir().join("spriter_test_save_load");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.spriter.toml");

    let project = Project {
        name: "test".to_string(),
        sprite_sheet: PathBuf::from("sheet.png"),
        frame_width: 32,
        frame_height: 32,
        presets: vec![Preset {
            name: "default".to_string(),
            animations: vec![Animation {
                name: "idle".to_string(),
                frames: vec![0, 1, 2],
                fps: 8.0,
                looping: true,
            }],
        }],
    };

    save_project(&project, &path).unwrap();
    let loaded = load_project(&path).unwrap();

    assert_eq!(loaded.name, "test");
    assert_eq!(loaded.presets[0].animations[0].frames, vec![0, 1, 2]);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_nonexistent_file_returns_error() {
    let result = load_project(std::path::Path::new("/nonexistent/path.toml"));
    assert!(result.is_err());
}

#[test]
fn test_load_invalid_toml_returns_error() {
    let dir = std::env::temp_dir().join("spriter_test_invalid_toml");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bad.spriter.toml");
    std::fs::write(&path, "this is not valid toml [[[").unwrap();

    let result = load_project(&path);
    assert!(result.is_err());

    std::fs::remove_dir_all(&dir).ok();
}
