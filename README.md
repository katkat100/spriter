# Spriter

A pixel art sprite sheet animator built with Rust and egui.

## Features

- **Sprite Sheet Loading**: Load PNG, JPG, or BMP sprite sheets via drag-and-drop or file dialog
- **Frame Extraction**: Automatically split sprite sheets into individual frames based on configurable frame dimensions
- **Animation Editor**: Create and organize animations with customizable frame sequences
- **Presets**: Group related animations into presets for easy organization
- **Playback Controls**: Play, pause, step through frames with adjustable FPS
- **Loop & Ping-Pong**: Support for looping and ping-pong animation modes
- **GIF Export**: Export animations as GIF files with configurable scale
- **Project Files**: Save and load projects as `.spriter.toml` files
- **Multi-Tab Support**: Work on multiple sprite sheets simultaneously
- **Session Restore**: Automatically reopens your last session

## Getting Started

### Running

```bash
cargo run
```

Or with a sprite sheet:

```bash
cargo run -- --image path/to/spritesheet.png --width 32 --height 32
```

Or open a project file:

```bash
cargo run -- path/to/project.spriter.toml
```

### Basic Workflow

1. **Load a sprite sheet**: Drag and drop an image onto the window, or click OPEN
2. **Set frame size**: Enter the width and height of each frame in your sprite sheet
3. **Create animations**: Click "+ ANIMATION" and select frames from the grid
4. **Adjust playback**: Set FPS, enable looping or ping-pong mode
5. **Save your project**: Click SAVE to create a `.spriter.toml` project file
6. **Export**: Use the export options to create GIFs

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Space` | Play/Pause |
| `←` / `→` | Step backward/forward one frame |
| `Cmd/Ctrl+R` | Refresh sprite sheet from disk |

### Refreshing Sprite Sheets

If you edit your source image in an external editor (Aseprite, Photoshop, etc.), you can reload it in Spriter without losing your animation setup:

- Press `Cmd+R` (macOS) or `Ctrl+R` (Windows/Linux)
- Or click the REFRESH button below the sprite sheet preview

## Project Structure

Projects are saved as TOML files containing:

- Reference to the sprite sheet image (relative path)
- Frame dimensions
- Presets and their animations
- Frame sequences, FPS, and playback settings

## Building

```bash
cargo build --release
```

## License

MIT
