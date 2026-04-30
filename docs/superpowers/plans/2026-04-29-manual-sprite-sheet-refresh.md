# Manual Sprite Sheet Refresh Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add manual sprite sheet refresh via keyboard shortcut (Cmd/Ctrl+R) and side panel button.

**Architecture:** Add a `refresh_sprite_sheet` method to `Tab` that reloads the sprite sheet from disk using existing frame dimensions. Wire it to a keyboard shortcut in `App::update()` and a button in the side panel.

**Tech Stack:** Rust, egui, existing `SpriteSheet::load_frames()`

---

## File Structure

| File | Changes |
|------|---------|
| `src/tab.rs` | Add `refresh_sprite_sheet(&mut self, ctx: &egui::Context) -> Result<(), String>` method |
| `src/app.rs` | Add keyboard shortcut handler for Cmd/Ctrl+R in `update()` |
| `src/ui/side_panel.rs` | Add REFRESH button near sprite sheet preview |

---

### Task 1: Add `refresh_sprite_sheet` Method to Tab

**Files:**
- Modify: `src/tab.rs:43-91` (after `load_sprite_sheet` method)

- [ ] **Step 1: Add the `refresh_sprite_sheet` method**

Add this method to the `Tab` impl block after `load_sprite_sheet`:

```rust
pub fn refresh_sprite_sheet(&mut self, ctx: &egui::Context) -> Result<(), String> {
    // Return early if no sprite sheet loaded
    let sheet = match &self.sheet {
        Some(s) => s,
        None => return Ok(()),
    };

    // Get current frame dimensions
    let frame_width = sheet.frame_width;
    let frame_height = sheet.frame_height;

    // Get the sprite sheet path
    let path = &self.project.sprite_sheet;
    if path.as_os_str().is_empty() {
        return Ok(());
    }

    // Reload the sprite sheet
    match SpriteSheet::load_frames(path, frame_width, frame_height) {
        Ok((new_sheet, full_image, frames)) => {
            self.sheet_texture = Some(ctx.load_texture(
                "sprite_sheet",
                full_image,
                egui::TextureOptions::NEAREST,
            ));
            self.frame_textures = frames.into_iter().enumerate().map(|(i, img)| {
                ctx.load_texture(
                    format!("frame_{i}"),
                    img,
                    egui::TextureOptions::NEAREST,
                )
            }).collect();
            self.sheet = Some(new_sheet);
            Ok(())
        }
        Err(e) => Err(format!("Failed to refresh: {e}")),
    }
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo build`
Expected: Build succeeds with no errors

- [ ] **Step 3: Commit**

```bash
git add src/tab.rs
git commit -m "feat(tab): add refresh_sprite_sheet method"
```

---

### Task 2: Add Keyboard Shortcut Handler

**Files:**
- Modify: `src/app.rs:138-148` (in `update()` method, after playback advance block)

- [ ] **Step 1: Add keyboard shortcut handler after the playback block**

Insert this code after line 149 (after the playback block closing brace) and before the dropped files handling:

```rust
// Handle Cmd/Ctrl+R to refresh sprite sheet
if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::R)) {
    let tab = &mut self.tabs[self.active_tab];
    match tab.refresh_sprite_sheet(ctx) {
        Ok(()) => {
            if tab.sheet.is_some() {
                self.status_message = Some("Sprite sheet refreshed".to_string());
            }
        }
        Err(e) => self.error_message = Some(e),
    }
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo build`
Expected: Build succeeds with no errors

- [ ] **Step 3: Test manually**

Run: `cargo run`
1. Open a sprite sheet
2. Modify the source image in an external editor and save
3. Press Cmd+R (macOS) or Ctrl+R (Windows/Linux)
Expected: Sprite sheet reloads, status message "Sprite sheet refreshed" appears

- [ ] **Step 4: Commit**

```bash
git add src/app.rs
git commit -m "feat(app): add Cmd/Ctrl+R shortcut to refresh sprite sheet"
```

---

### Task 3: Add Refresh Button to Side Panel

**Files:**
- Modify: `src/ui/side_panel.rs:119-127` (after sprite sheet preview section)

- [ ] **Step 1: Add REFRESH button after the sprite sheet preview**

Replace lines 119-127 with:

```rust
// Sprite sheet preview
let sheet_tex_id = app.tabs[app.active_tab].sheet_texture.as_ref().map(|t| (t.id(), t.size_vec2()));
if let Some((tex_id, tex_size)) = sheet_tex_id {
    ui.add_space(4.0);
    let panel_width = ui.available_width();
    let scale = panel_width / tex_size.x;
    let display_size = Vec2::new(panel_width, tex_size.y * scale);
    ui.image(egui::load::SizedTexture::new(tex_id, display_size));

    // Refresh button
    if retro_button(ui, "REFRESH").clicked() {
        let tab = &mut app.tabs[app.active_tab];
        match tab.refresh_sprite_sheet(ui.ctx()) {
            Ok(()) => app.status_message = Some("Sprite sheet refreshed".to_string()),
            Err(e) => app.error_message = Some(e),
        }
    }
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo build`
Expected: Build succeeds with no errors

- [ ] **Step 3: Test manually**

Run: `cargo run`
1. Open a sprite sheet - REFRESH button should appear below preview
2. Click REFRESH button
Expected: Status message "Sprite sheet refreshed" appears

- [ ] **Step 4: Commit**

```bash
git add src/ui/side_panel.rs
git commit -m "feat(side-panel): add REFRESH button for sprite sheet"
```

---

### Task 4: Final Verification

- [ ] **Step 1: Run full build**

Run: `cargo build`
Expected: Build succeeds

- [ ] **Step 2: Run tests**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 3: End-to-end manual test**

Run: `cargo run`
1. Open a sprite sheet image
2. Press Cmd/Ctrl+R - should show "Sprite sheet refreshed"
3. Verify REFRESH button is visible below sprite sheet preview
4. Click REFRESH button - should show "Sprite sheet refreshed"
5. Close the app and reopen with no sprite sheet - REFRESH button should not appear
6. Press Cmd/Ctrl+R with no sprite sheet - nothing should happen (no error)

- [ ] **Step 4: Commit all changes if not already committed**

```bash
git status
# If any uncommitted changes:
git add -A
git commit -m "feat: manual sprite sheet refresh via Cmd/Ctrl+R and button"
```
