use eframe::egui;

use crate::config::AppConfig;
use crate::tab::Tab;

pub struct Spriter {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub show_frame_size_dialog: bool,
    pub frame_size_input: [String; 2],
    pub dark_bg: bool,
    pub export_scale: u32,
    pub config: AppConfig,
}

impl Spriter {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        apply_retro_theme(&cc.egui_ctx);
        Self {
            tabs: vec![Tab::new()],
            active_tab: 0,
            error_message: None,
            status_message: None,
            show_frame_size_dialog: false,
            frame_size_input: ["32".to_string(), "32".to_string()],
            dark_bg: true,
            export_scale: 1,
            config,
        }
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.active_tab]
    }

    pub fn active_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab]
    }

    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            // Replace with empty tab instead of removing last
            self.tabs[0] = Tab::new();
            self.active_tab = 0;
            return;
        }
        self.tabs.remove(index);
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        } else if self.active_tab > index {
            self.active_tab -= 1;
        }
    }

    pub fn open_project_file(&mut self, path: &std::path::Path, _ctx: &egui::Context) {
        match crate::io::project_file::load_project(path) {
            Ok(project) => {
                let mut tab = Tab::new();
                tab.project = project;
                tab.save_path = Some(path.to_path_buf());
                let sheet_path = path.parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join(&tab.project.sprite_sheet);
                tab.pending_sheet_load = Some(sheet_path);
                self.add_tab(tab);
                self.config.add_recent(path.to_path_buf());
                self.config.save();
            }
            Err(e) => self.error_message = Some(e),
        }
    }

    pub fn open_image_file(&mut self, path: &std::path::Path) {
        let mut tab = Tab::new();
        tab.project.sprite_sheet = path.to_path_buf();
        self.add_tab(tab);
        self.show_frame_size_dialog = true;
        self.frame_size_input = [
            self.active_tab().project.frame_width.to_string(),
            self.active_tab().project.frame_height.to_string(),
        ];
    }

    pub fn save_session(&mut self) {
        let open_paths: Vec<std::path::PathBuf> = self.tabs.iter()
            .filter_map(|t| t.save_path.clone())
            .collect();
        self.config.open_files = open_paths;
        self.config.active_index = self.active_tab;
        self.config.save();
    }

    pub fn restore_session(&mut self, _ctx: &egui::Context) {
        let paths: Vec<std::path::PathBuf> = self.config.open_files.clone();
        let active = self.config.active_index;

        if paths.is_empty() {
            return;
        }

        // Remove the default empty tab
        self.tabs.clear();

        for path in &paths {
            if path.exists() {
                match crate::io::project_file::load_project(path) {
                    Ok(project) => {
                        let mut tab = Tab::new();
                        let sheet_path = path.parent()
                            .unwrap_or(std::path::Path::new("."))
                            .join(&project.sprite_sheet);
                        tab.project = project;
                        tab.save_path = Some(path.clone());
                        tab.pending_sheet_load = Some(sheet_path);
                        self.tabs.push(tab);
                    }
                    Err(_) => {}
                }
            }
        }

        if self.tabs.is_empty() {
            self.tabs.push(Tab::new());
        }
        self.active_tab = active.min(self.tabs.len() - 1);
    }
}

impl eframe::App for Spriter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Advance playback on active tab
        {
            let tab = &mut self.tabs[self.active_tab];
            if tab.playback.playing {
                ctx.request_repaint();
                if let Some(anim) = tab.current_animation().cloned() {
                    let dt = ctx.input(|i| i.stable_dt);
                    tab.playback.advance(dt, anim.fps, anim.frames.len(), anim.looping, anim.ping_pong);
                }
            }
        }

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

        // Handle dropped files
        let dropped: Vec<_> = ctx.input(|i| i.raw.dropped_files.clone());
        for file in &dropped {
            if let Some(path) = &file.path {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                match ext {
                    "toml" => {
                        self.open_project_file(path, ctx);
                    }
                    "png" | "jpg" | "jpeg" | "bmp" => {
                        self.open_image_file(path);
                    }
                    _ => {
                        self.error_message = Some(format!("Unsupported file type: .{ext}"));
                    }
                }
            }
        }

        // Handle deferred sprite sheet load on active tab
        let pending = self.tabs[self.active_tab].pending_sheet_load.take();
        if let Some(path) = pending {
            if path.exists() {
                match self.tabs[self.active_tab].load_sprite_sheet(&path, ctx) {
                    Ok(()) => {}
                    Err(msg) => self.status_message = Some(msg),
                }
            } else {
                self.error_message = Some(format!(
                    "Sprite sheet not found: {}. Use Open to locate it.",
                    path.display()
                ));
            }
        }

        // Also process pending loads on non-active tabs (for session restore)
        for i in 0..self.tabs.len() {
            if i == self.active_tab { continue; }
            let pending = self.tabs[i].pending_sheet_load.take();
            if let Some(path) = pending {
                if path.exists() {
                    let _ = self.tabs[i].load_sprite_sheet(&path, ctx);
                }
            }
        }

        if self.show_frame_size_dialog {
            egui::Window::new("Frame Size")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("Enter frame dimensions for this sprite sheet:");
                    ui.horizontal(|ui| {
                        ui.label("Width:");
                        ui.text_edit_singleline(&mut self.frame_size_input[0]);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Height:");
                        ui.text_edit_singleline(&mut self.frame_size_input[1]);
                    });
                    ui.horizontal(|ui| {
                        if crate::ui::widgets::retro_button(ui, "OK").clicked()
                            && let (Ok(w), Ok(h)) = (
                                self.frame_size_input[0].trim().parse::<u32>(),
                                self.frame_size_input[1].trim().parse::<u32>(),
                            )
                        {
                            let tab = &mut self.tabs[self.active_tab];
                            tab.project.frame_width = w;
                            tab.project.frame_height = h;
                            tab.pending_sheet_load = Some(tab.project.sprite_sheet.clone());
                            self.show_frame_size_dialog = false;
                        }
                        if crate::ui::widgets::retro_button(ui, "CANCEL").clicked() {
                            self.show_frame_size_dialog = false;
                        }
                    });
                });
        }

        // Tab bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            crate::ui::tab_bar::show(ui, self);
        });

        egui::SidePanel::left("side_panel")
            .default_width(220.0)
            .show(ctx, |ui| {
                crate::ui::side_panel::show(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::canvas::show(ui, self);
            ui.separator();
            crate::ui::playback_bar::show(ui, self);
        });
    }
}

fn apply_retro_theme(ctx: &egui::Context) {
    use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke, style::Widgets, style::WidgetVisuals};

    // Load pixel font
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "pixel".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(
            include_bytes!("../assets/fonts/VT323-Regular.ttf"),
        )),
    );
    // Set as primary for both proportional and monospace
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "pixel".to_owned());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "pixel".to_owned());
    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();

    // Text sizes for VT323
    style.text_styles.insert(egui::TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Body, FontId::new(16.0, FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Button, FontId::new(16.0, FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Small, FontId::new(14.0, FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Monospace, FontId::new(16.0, FontFamily::Monospace));

    // Square everything off for pixel-art feel
    let no_round = CornerRadius::ZERO;

    // Muted retro palette
    let bg_dark = Color32::from_rgb(30, 30, 36);
    let bg_panel = Color32::from_rgb(38, 38, 46);
    let bg_widget = Color32::from_rgb(50, 50, 60);
    let bg_widget_hover = Color32::from_rgb(65, 65, 80);
    let bg_widget_active = Color32::from_rgb(80, 80, 100);
    let accent = Color32::from_rgb(110, 160, 220);
    let text_color = Color32::from_rgb(210, 210, 220);
    let text_dim = Color32::from_rgb(140, 140, 155);

    style.visuals.dark_mode = true;
    style.visuals.panel_fill = bg_panel;
    style.visuals.window_fill = bg_panel;
    style.visuals.extreme_bg_color = bg_dark;
    style.visuals.faint_bg_color = Color32::from_rgb(42, 42, 52);
    style.visuals.override_text_color = Some(text_color);

    style.visuals.selection.bg_fill = accent.gamma_multiply(0.4);
    style.visuals.selection.stroke = Stroke::new(1.0, accent);

    style.visuals.window_corner_radius = no_round;
    style.visuals.menu_corner_radius = no_round;

    style.visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 72));
    style.visuals.window_shadow = egui::epaint::Shadow::NONE;
    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;

    // Widget styles
    let noninteractive = WidgetVisuals {
        bg_fill: bg_panel,
        weak_bg_fill: bg_panel,
        bg_stroke: Stroke::new(1.0, Color32::from_rgb(55, 55, 65)),
        corner_radius: no_round,
        fg_stroke: Stroke::new(1.0, text_dim),
        expansion: 0.0,
    };
    let inactive = WidgetVisuals {
        bg_fill: bg_widget,
        weak_bg_fill: bg_widget,
        bg_stroke: Stroke::new(1.0, Color32::from_rgb(65, 65, 78)),
        corner_radius: no_round,
        fg_stroke: Stroke::new(1.0, text_color),
        expansion: 0.0,
    };
    let hovered = WidgetVisuals {
        bg_fill: bg_widget_hover,
        weak_bg_fill: bg_widget_hover,
        bg_stroke: Stroke::new(1.0, accent),
        corner_radius: no_round,
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 0.0,
    };
    let active = WidgetVisuals {
        bg_fill: bg_widget_active,
        weak_bg_fill: bg_widget_active,
        bg_stroke: Stroke::new(1.5, accent),
        corner_radius: no_round,
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 0.0,
    };
    let open = WidgetVisuals {
        bg_fill: bg_widget_active,
        weak_bg_fill: bg_widget_active,
        bg_stroke: Stroke::new(1.0, accent),
        corner_radius: no_round,
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 0.0,
    };

    style.visuals.widgets = Widgets {
        noninteractive,
        inactive,
        hovered,
        active,
        open,
    };

    // Tighter spacing for compact retro feel
    style.spacing.item_spacing = egui::vec2(6.0, 4.0);
    style.spacing.button_padding = egui::vec2(6.0, 3.0);
    style.spacing.window_margin = egui::Margin::same(6);

    ctx.set_style(style);
}
