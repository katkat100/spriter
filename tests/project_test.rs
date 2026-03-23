use spriter::model::project::{Animation, Preset, Project};
use std::path::PathBuf;

#[test]
fn test_project_toml_round_trip() {
    let project = Project {
        name: "test-project".to_string(),
        sprite_sheet: PathBuf::from("sprites/hero.png"),
        frame_width: 64,
        frame_height: 64,
        presets: vec![Preset {
            name: "default".to_string(),
            animations: vec![Animation {
                name: "walk".to_string(),
                frames: vec![0, 1, 2, 3],
                fps: 12.0,
                looping: true,
            }],
        }],
    };

    let toml_str = toml::to_string_pretty(&project).unwrap();
    let loaded: Project = toml::from_str(&toml_str).unwrap();

    assert_eq!(loaded.name, "test-project");
    assert_eq!(loaded.sprite_sheet, PathBuf::from("sprites/hero.png"));
    assert_eq!(loaded.frame_width, 64);
    assert_eq!(loaded.presets.len(), 1);
    assert_eq!(loaded.presets[0].animations[0].name, "walk");
    assert_eq!(loaded.presets[0].animations[0].frames, vec![0, 1, 2, 3]);
}

#[test]
fn test_project_default_has_empty_presets() {
    let project = Project::default();
    assert!(project.presets.is_empty());
    assert_eq!(project.frame_width, 32);
    assert_eq!(project.frame_height, 32);
}
