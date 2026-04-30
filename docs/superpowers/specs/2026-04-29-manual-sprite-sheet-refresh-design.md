# Manual Sprite Sheet Refresh

**Date**: 2026-04-29

## Overview

Add ability to manually reload the current tab's sprite sheet from disk, preserving frame dimensions. Useful when editing the source image in an external application.

## Triggers

- **Keyboard shortcut**: `Cmd+R` (macOS) / `Ctrl+R` (Windows/Linux)
- **Button**: "Refresh" button in side panel, near the sprite sheet thumbnail

## Behavior

1. Read the sprite sheet path from `project.sprite_sheet`
2. Re-run `SpriteSheet::load_frames()` with existing frame width/height from current `SpriteSheet`
3. Replace `sheet_texture` and `frame_textures` with new GPU textures
4. Keep all other state intact (animations, playback position, project settings)
5. If image dimensions changed, recalculate grid (new row/column count) using existing frame size

## Edge Cases

| Scenario | Behavior |
|----------|----------|
| File missing | Show error message, keep existing textures |
| No sprite sheet loaded | Button disabled, shortcut does nothing |
| Image read error | Show error message, keep existing textures |
| Image dimensions changed | Recalculate grid with same frame size |

## Implementation

### New Method: `Tab::refresh_sprite_sheet`

Add to `src/tab.rs`:

```rust
pub fn refresh_sprite_sheet(&mut self, ctx: &egui::Context) -> Result<(), String>
```

This method:
- Returns early with `Ok(())` if no sprite sheet is loaded
- Gets frame dimensions from existing `self.sheet`
- Calls `SpriteSheet::load_frames()` with current frame size
- Replaces texture handles with new ones
- Returns `Err(message)` on failure

### Keyboard Handler

In `src/app.rs` `App::update()`:
- Check for `Cmd/Ctrl+R` input
- Call `active_tab.refresh_sprite_sheet(ctx)`
- Display error if result is `Err`

### Side Panel Button

In `src/ui/side_panel.rs`:
- Add "Refresh" button near sprite sheet thumbnail
- Disable when no sprite sheet loaded
- On click, call `tab.refresh_sprite_sheet(ctx)`
- Display error if result is `Err`

## Files Modified

- `src/tab.rs` - Add `refresh_sprite_sheet` method
- `src/app.rs` - Add keyboard shortcut handler
- `src/ui/side_panel.rs` - Add refresh button
