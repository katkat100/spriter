# Spriter — Sprite Animation Viewer Design

## Overview

Spriter is a Rust-based sprite animation viewer for game development. It allows developers to load grid-based sprite sheets, define and preview animations visually, and save/load animation configurations as project files with multiple named presets.

## Goals

- Preview sprite animations quickly during game dev iteration
- Support multiple input methods: CLI argument, file dialog, drag & drop
- Define animations visually by selecting frame ranges from a grid-based sprite sheet
- Save/load project files containing multiple named presets per sprite sheet
- Ship with solid playback controls, designed for future extensibility (onion skinning, ping-pong, zoom/pan)

## Architecture

**Framework:** egui/eframe — immediate-mode GUI, single crate for UI + rendering, cross-platform.

### Components

- **App Shell (eframe)** — main window, event loop, state management
- **Side Panel** — project browser, preset selector, animation list with frame range editor
- **Sprite Canvas** — renders current animation frame on a checkerboard transparency background
- **Playback Controls** — play, pause, step forward/back, adjustable FPS slider, loop toggle

### Data Model

```rust
struct Project {
    name: String,
    sprite_sheet: PathBuf,  // relative to project file
    frame_width: u32,
    frame_height: u32,
    presets: Vec<Preset>,
}

struct Preset {
    name: String,
    animations: Vec<Animation>,
}

struct Animation {
    name: String,
    frames: Vec<usize>,     // frame indices into the grid
    fps: f32,
    looping: bool,
}
```

Columns are auto-calculated: `image_width / frame_width`.

### Project File Format

TOML format (`.spriter.toml`), human-readable and hand-editable. Sprite sheet paths are stored relative to the project file for portability.

```toml
name = "hero-character"
sprite_sheet = "hero-spritesheet.png"
frame_width = 64
frame_height = 64

[[presets]]
name = "8-direction"

  [[presets.animations]]
  name = "walk-down"
  frames = [0, 1, 2, 3, 4, 5, 6, 7]
  fps = 12.0
  looping = true

  [[presets.animations]]
  name = "walk-up"
  frames = [8, 9, 10, 11, 12, 13, 14, 15]
  fps = 12.0
  looping = true

[[presets]]
name = "4-direction"

  [[presets.animations]]
  name = "walk-down"
  frames = [0, 1, 2, 3, 4, 5, 6, 7]
  fps = 10.0
  looping = true
```

## UI Layout

- **Left panel (220px):** Project info (name, file controls) → Preset selector (list with active highlight) → Animation list (name + frame range, add/delete)
- **Center area:** Sprite canvas with checkerboard background, current frame rendered at display size with frame counter
- **Bottom bar:** Transport controls (first, prev, play/pause, next, last) | FPS slider | Loop toggle

## Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe` / `egui` | Window, GUI, rendering |
| `image` | Load PNG/BMP/JPEG, slice into frames |
| `rfd` | Native file dialog (open/save) |
| `serde` + `toml` | Serialize/deserialize project files |
| `clap` | CLI argument parsing |

## Input Methods

- **CLI:** `spriter mysheet.png` or `spriter project.spriter.toml`
- **File dialog:** Open button in the side panel, native OS dialog via `rfd`
- **Drag & drop:** Drop `.png` → new sprite sheet (prompt for frame dimensions). Drop `.spriter.toml` → load project.

## Error Handling

- **Image loading failures** — error dialog in UI, let user retry
- **Invalid project files** — parse errors displayed in-app with the problematic field
- **Missing sprite sheet** — prompt user to relocate the file
- **Frame size mismatch** — warn if image dimensions aren't evenly divisible by frame size, show clipped pixel count

## Future Extensibility

Designed but not implemented in v1:
- Ping-pong playback mode
- Onion skinning (ghost of previous/next frames)
- Zoom and pan on the sprite canvas
- Packed atlas support with metadata files
