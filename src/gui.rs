// src/gui.rs
use eframe::egui;
use egui::{Color32, RichText, Rounding, Stroke};
use std::fs;
use std::path::PathBuf;


mod theme {
    use egui::Color32;

    pub const BG_APP: Color32 = Color32::from_rgb(16, 18, 26);
    pub const BG_PANEL: Color32 = Color32::from_rgb(21, 24, 34);
    pub const BG_PANEL_ALT: Color32 = Color32::from_rgb(25, 29, 41);

    pub const BG_ROW_ALT: Color32 = Color32::from_rgb(26, 30, 43);
    pub const BG_WIDGET: Color32 = Color32::from_rgb(30, 34, 48);
    pub const BG_WIDGET_HOVER: Color32 = Color32::from_rgb(38, 43, 60);

    pub const BORDER: Color32 = Color32::from_rgb(40, 45, 60);

    pub const ACCENT: Color32 = Color32::from_rgb(56, 145, 255);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(94, 212, 230);
    pub const ACCENT_GOLD: Color32 = Color32::from_rgb(255, 196, 60);

    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(226, 230, 240);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(135, 143, 163);
    pub const TEXT_FAINT: Color32 = Color32::from_rgb(90, 97, 115);

    pub const SUCCESS: Color32 = Color32::from_rgb(90, 210, 130);
    pub const WARNING: Color32 = Color32::from_rgb(240, 190, 80);
    pub const DANGER: Color32 = Color32::from_rgb(235, 90, 90);

    pub const LIGHT_BG_APP: Color32 = Color32::from_rgb(246, 247, 250);
    pub const LIGHT_BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_BORDER: Color32 = Color32::from_rgb(222, 225, 232);
    pub const LIGHT_TEXT: Color32 = Color32::from_rgb(30, 33, 40);
}

/// Applies the AtomEX visual style (dark or light) to the egui context.
fn apply_theme(ctx: &egui::Context, dark: bool) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(6.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.indent = 14.0;

    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    if dark {
        visuals.override_text_color = Some(theme::TEXT_PRIMARY);
        visuals.panel_fill = theme::BG_PANEL;
        visuals.window_fill = theme::BG_PANEL;
        visuals.extreme_bg_color = theme::BG_APP;
        visuals.faint_bg_color = theme::BG_ROW_ALT;
        visuals.code_bg_color = theme::BG_WIDGET;
        visuals.hyperlink_color = theme::ACCENT_CYAN;
        visuals.selection.bg_fill = theme::ACCENT.linear_multiply(0.55);
        visuals.selection.stroke = Stroke::new(1.0, theme::ACCENT);

        visuals.widgets.noninteractive.bg_fill = theme::BG_PANEL;
        visuals.widgets.noninteractive.weak_bg_fill = theme::BG_PANEL;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme::BORDER);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme::TEXT_MUTED);
        visuals.widgets.noninteractive.rounding = Rounding::same(8.0);

        visuals.widgets.inactive.bg_fill = theme::BG_WIDGET;
        visuals.widgets.inactive.weak_bg_fill = theme::BG_WIDGET;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, theme::BORDER);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme::TEXT_PRIMARY);
        visuals.widgets.inactive.rounding = Rounding::same(8.0);

        visuals.widgets.hovered.bg_fill = theme::BG_WIDGET_HOVER;
        visuals.widgets.hovered.weak_bg_fill = theme::BG_WIDGET_HOVER;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, theme::ACCENT);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, theme::TEXT_PRIMARY);
        visuals.widgets.hovered.rounding = Rounding::same(8.0);

        visuals.widgets.active.bg_fill = theme::ACCENT;
        visuals.widgets.active.weak_bg_fill = theme::ACCENT;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, theme::ACCENT);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.rounding = Rounding::same(8.0);

        visuals.widgets.open.bg_fill = theme::BG_WIDGET_HOVER;
        visuals.widgets.open.weak_bg_fill = theme::BG_WIDGET_HOVER;
        visuals.widgets.open.rounding = Rounding::same(8.0);

        visuals.window_rounding = Rounding::same(10.0);
        visuals.window_stroke = Stroke::new(1.0, theme::BORDER);
        visuals.menu_rounding = Rounding::same(8.0);
        visuals.indent_has_left_vline = false;
        visuals.striped = false;
    } else {
        visuals.panel_fill = theme::LIGHT_BG_PANEL;
        visuals.window_fill = theme::LIGHT_BG_PANEL;
        visuals.extreme_bg_color = theme::LIGHT_BG_APP;
        visuals.override_text_color = Some(theme::LIGHT_TEXT);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme::LIGHT_BORDER);
        visuals.selection.bg_fill = theme::ACCENT.linear_multiply(0.35);
        for w in [
            &mut visuals.widgets.inactive,
            &mut visuals.widgets.hovered,
            &mut visuals.widgets.active,
            &mut visuals.widgets.open,
        ] {
            w.rounding = Rounding::same(8.0);
        }
    }

    style.visuals = visuals;
    ctx.set_style(style);
}

/// A small on/off pill switch, used for things like the endianness setting.
fn toggle_switch(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = egui::vec2(38.0, 20.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let t = ui.ctx().animate_bool(response.id, *on);
        let bg = if *on {
            theme::ACCENT
        } else {
            theme::BG_WIDGET_HOVER
        };
        let radius = rect.height() / 2.0;
        ui.painter().rect_filled(rect, radius, bg);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), t);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter().circle_filled(center, radius - 3.0, Color32::WHITE);
    }
    response
}

/// Small colored "pill" label, used for the status bar segments.
fn pill_label(ui: &mut egui::Ui, text: impl Into<String>, color: Color32) {
    egui::Frame::none()
        .fill(theme::BG_WIDGET)
        .rounding(Rounding::same(6.0))
        .inner_margin(egui::Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text.into()).small().color(color));
        });
}

#[derive(Default, Clone)]
struct EditOperation {
    line: usize,
    offset: usize,
    old_data: Vec<u8>,
    new_data: Vec<u8>,
    timestamp: u64,
}

pub struct MyApp {
    // File data.
    file_path: Option<PathBuf>,
    file_data: Vec<u8>,
    original_data: Vec<u8>,

    // UI state.
    selected_offset: usize,
    bytes_per_row: usize,
    show_data_inspector: bool,
    show_search_dialog: bool,
    show_go_to_dialog: bool,
    show_edit_dialog: bool,
    dark_mode: bool,

    // Search.
    search_query: String,
    search_results: Vec<usize>,
    current_search_index: usize,

    // Edit dialog.
    edit_line: String,
    edit_offset: String,
    edit_data: String,

    // Go to.
    go_to_offset: String,

    // Settings.
    character_encoding: Encoding,
    endianness: Endianness,

    // Undo/Redo.
    undo_stack: Vec<EditOperation>,
    redo_stack: Vec<EditOperation>,
    operation_counter: u64,

    // Recent files.
    recent_files: Vec<PathBuf>,

    // Bookmarks (byte offsets the user has flagged).
    bookmarks: Vec<usize>,

    // Status.
    status_message: String,
    status_color: Color32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_path: None,
            file_data: Vec::new(),
            original_data: Vec::new(),
            selected_offset: 0,
            bytes_per_row: 16,
            show_data_inspector: true,
            show_search_dialog: false,
            show_go_to_dialog: false,
            show_edit_dialog: false,
            dark_mode: true,
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: 0,
            edit_line: String::new(),
            edit_offset: String::new(),
            edit_data: String::new(),
            go_to_offset: String::new(),
            character_encoding: Encoding::default(),
            endianness: Endianness::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            operation_counter: 0,
            recent_files: Vec::new(),
            bookmarks: Vec::new(),
            status_message: String::new(),
            status_color: Color32::GRAY,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Encoding {
    ASCII,
    UTF8,
    UTF16,
}

#[derive(PartialEq, Clone, Copy)]
enum Endianness {
    Little,
    Big,
}

impl Default for Encoding {
    fn default() -> Self {
        Self::ASCII
    }
}

impl Default for Endianness {
    fn default() -> Self {
        Self::Little
    }
}

impl Encoding {
    fn label(&self) -> &'static str {
        match self {
            Encoding::ASCII => "ASCII",
            Encoding::UTF8 => "UTF-8",
            Encoding::UTF16 => "UTF-16",
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_theme(&cc.egui_ctx, true);

        let mut app = MyApp::default();
        app.bytes_per_row = 16;
        app.show_data_inspector = true;
        app.status_message = "Ready".to_string();
        app.status_color = theme::TEXT_MUTED;

        app
    }

    fn load_file(&mut self, path: PathBuf) {
        match fs::read(&path) {
            Ok(data) => {
                // Store the path for display after moving.
                let path_display = path.display().to_string();
                self.file_path = Some(path);
                self.file_data = data.clone();
                self.original_data = data;
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.selected_offset = 0;
                self.bookmarks.clear();

                // Add to recent files (clone before moving).
                let path_clone = PathBuf::from(&path_display);
                if !self.recent_files.contains(&path_clone) {
                    self.recent_files.insert(0, path_clone);
                    if self.recent_files.len() > 10 {
                        self.recent_files.pop();
                    }
                }

                self.status_message = format!("Loaded: {}", path_display);
                self.status_color = theme::SUCCESS;
            }
            Err(e) => {
                self.status_message = format!("Error loading file: {}", e);
                self.status_color = theme::DANGER;
            }
        }
    }

    fn close_file(&mut self) {
        self.file_path = None;
        self.file_data.clear();
        self.original_data.clear();
        self.selected_offset = 0;
        self.bookmarks.clear();
        self.status_message = "Closed file".to_string();
        self.status_color = theme::TEXT_MUTED;
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            match fs::write(path, &self.file_data) {
                Ok(_) => {
                    self.original_data = self.file_data.clone();
                    self.status_message = format!("Saved: {}", path.display());
                    self.status_color = theme::SUCCESS;
                }
                Err(e) => {
                    self.status_message = format!("Error saving: {}", e);
                    self.status_color = theme::DANGER;
                }
            }
        }
    }

    fn toggle_bookmark(&mut self, offset: usize) {
        if let Some(pos) = self.bookmarks.iter().position(|&o| o == offset) {
            self.bookmarks.remove(pos);
            self.status_message = format!("Removed bookmark at 0x{:08X}", offset);
        } else {
            self.bookmarks.push(offset);
            self.bookmarks.sort_unstable();
            self.status_message = format!("Bookmarked 0x{:08X}", offset);
        }
        self.status_color = theme::ACCENT_GOLD;
    }

    fn perform_search(&mut self) {
        self.search_results.clear();
        if self.search_query.is_empty() || self.file_data.is_empty() {
            return;
        }

        // Try to parse as hex first.
        let query_bytes = if self.search_query.contains(' ') {
            // Try hex bytes
            self.search_query
                .split_whitespace()
                .filter_map(|s| u8::from_str_radix(s, 16).ok())
                .collect::<Vec<u8>>()
        } else {
            // Try as ASCII or hex string.
            if self.search_query.starts_with("0x")
                || self.search_query.chars().all(|c| c.is_ascii_hexdigit())
            {
                let hex = self.search_query.trim_start_matches("0x");
                if hex.len() % 2 == 0 {
                    (0..hex.len())
                        .step_by(2)
                        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
                        .collect::<Vec<u8>>()
                } else {
                    self.search_query.as_bytes().to_vec()
                }
            } else {
                self.search_query.as_bytes().to_vec()
            }
        };

        if query_bytes.is_empty() {
            return;
        }

        for (i, window) in self.file_data.windows(query_bytes.len()).enumerate() {
            if window == query_bytes {
                self.search_results.push(i);
            }
        }

        self.current_search_index = 0;
        if !self.search_results.is_empty() {
            self.selected_offset = self.search_results[0];
            self.status_message = format!("Found {} results", self.search_results.len());
            self.status_color = theme::ACCENT_CYAN;
        } else {
            self.status_message = "No results found".to_string();
            self.status_color = theme::WARNING;
        }
    }

    fn navigate_search(&mut self, direction: i32) {
        if self.search_results.is_empty() {
            return;
        }

        let new_index = (self.current_search_index as i32 + direction)
            .rem_euclid(self.search_results.len() as i32) as usize;
        self.current_search_index = new_index;
        self.selected_offset = self.search_results[new_index];
    }
    // Rendering.

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);

            // Logo mark.
            let (rect, _resp) = ui.allocate_exact_size(egui::vec2(26.0, 26.0), egui::Sense::hover());
            ui.painter()
                .rect_filled(rect, Rounding::same(7.0), theme::ACCENT);
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "A",
                egui::FontId::proportional(16.0),
                Color32::WHITE,
            );

            ui.add_space(6.0);
            let title = match &self.file_path {
                Some(p) => format!(
                    "AtomEX - {}",
                    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
                ),
                None => "AtomEX".to_string(),
            };
            ui.label(RichText::new(title).strong().size(15.0));
            ui.add_space(14.0);

            // File menu.
            ui.menu_button("File", |ui| {
                if ui.button("📂  Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.load_file(path);
                    }
                    ui.close_menu();
                }
                if ui.button("💾  Save").clicked() {
                    self.save_file();
                    ui.close_menu();
                }
                if ui.button("Save As").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        let _ = fs::write(path, &self.file_data);
                    }
                    ui.close_menu();
                }
                if ui.button("✕  Close File").clicked() {
                    self.close_file();
                    ui.close_menu();
                }
                ui.separator();

                // Recent files.
                if !self.recent_files.is_empty() {
                    ui.label(RichText::new("Recent Files").color(theme::TEXT_MUTED).small());
                    let recent_files: Vec<PathBuf> = self.recent_files.clone();
                    for path in &recent_files {
                        if ui.button(path.display().to_string()).clicked() {
                            self.load_file(path.clone());
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                }

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            // Edit menu.
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    ui.close_menu();
                }
                if ui.button("Redo").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    ui.close_menu();
                }
                if ui.button("Copy").clicked() {
                    ui.close_menu();
                }
                if ui.button("Paste").clicked() {
                    ui.close_menu();
                }
            });

            // View menu.
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut self.show_data_inspector, "Data Inspector");
                ui.separator();
                if ui.button("Refresh").clicked() {
                    ui.close_menu();
                }
            });

            // Search menu.
            ui.menu_button("Search", |ui| {
                if ui.button("Find...").clicked() {
                    self.show_search_dialog = true;
                    ui.close_menu();
                }
                if ui.button("Go To Offset...").clicked() {
                    self.show_go_to_dialog = true;
                    ui.close_menu();
                }
            });

            // Analyze menu.
            ui.menu_button("Analyze", |ui| {
                if ui.button("Find Patterns").clicked() {
                    ui.close_menu();
                }
                if ui.button("Check Integrity").clicked() {
                    ui.close_menu();
                }
            });

            // Tools / Settings menu.
            ui.menu_button("Tools", |ui| {
                ui.menu_button("Character Encoding", |ui| {
                    if ui.button("ASCII").clicked() {
                        self.character_encoding = Encoding::ASCII;
                        ui.close_menu();
                    }
                    if ui.button("UTF-8").clicked() {
                        self.character_encoding = Encoding::UTF8;
                        ui.close_menu();
                    }
                    if ui.button("UTF-16").clicked() {
                        self.character_encoding = Encoding::UTF16;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Endianness", |ui| {
                    if ui.button("Little Endian").clicked() {
                        self.endianness = Endianness::Little;
                        ui.close_menu();
                    }
                    if ui.button("Big Endian").clicked() {
                        self.endianness = Endianness::Big;
                        ui.close_menu();
                    }
                });
            });

            // Help menu.
            ui.menu_button("Help", |ui| {
                if ui.button("About AtomEX").clicked() {
                    ui.close_menu();
                }
            });
        });
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            if ui.button("📂  Open").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.load_file(path);
                }
            }
            if ui.button("💾  Save").clicked() {
                self.save_file();
            }

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            ui.add_enabled_ui(!self.undo_stack.is_empty(), |ui| {
                let _ = ui.button("↶  Undo");
            });
            ui.add_enabled_ui(!self.redo_stack.is_empty(), |ui| {
                let _ = ui.button("↷  Redo");
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            if ui.button("🔍  Search").clicked() {
                self.show_search_dialog = !self.show_search_dialog;
            }
            if ui.button("📍  Go To Offset").clicked() {
                self.show_go_to_dialog = !self.show_go_to_dialog;
            }
            if ui.button("✏  Edit").clicked() {
                self.show_edit_dialog = !self.show_edit_dialog;
            }
            let bookmark_label = if self.file_data.is_empty()
                || !self.bookmarks.contains(&self.selected_offset)
            {
                "🔖  Bookmark"
            } else {
                "🔖  Unbookmark"
            };
            ui.add_enabled_ui(!self.file_data.is_empty(), |ui| {
                if ui.button(bookmark_label).clicked() {
                    self.toggle_bookmark(self.selected_offset);
                }
            });

            // Right-aligned settings / theme controls.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let theme_label = if self.dark_mode { "🌙  Theme" } else { "☀  Theme" };
                if ui.button(theme_label).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
                if ui.button("⚙  Settings").clicked() {
                    self.show_data_inspector = !self.show_data_inspector;
                }
            });
        });
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.label(RichText::new("OPEN FILES").small().strong().color(theme::TEXT_MUTED));
        ui.add_space(4.0);

        if let Some(path) = self.file_path.clone() {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());

            egui::Frame::none()
                .fill(theme::BG_WIDGET)
                .rounding(Rounding::same(6.0))
                .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let stripe_rect = egui::Rect::from_min_size(
                            ui.cursor().min,
                            egui::vec2(3.0, 18.0),
                        );
                        ui.painter().rect_filled(stripe_rect, 2.0, theme::ACCENT);
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("📄 {}", name)).color(theme::TEXT_PRIMARY));
                    });
                });
        } else {
            ui.label(RichText::new("No files open").italics().color(theme::TEXT_FAINT));
        }

        ui.add_space(16.0);
        ui.label(RichText::new("RECENT FILES").small().strong().color(theme::TEXT_MUTED));
        ui.add_space(4.0);

        if self.recent_files.is_empty() {
            ui.label(RichText::new("Nothing yet").italics().color(theme::TEXT_FAINT));
        } else {
            let recent_files = self.recent_files.clone();
            for path in recent_files.iter().take(8) {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string());
                if ui
                    .add(egui::Label::new(format!("📄 {}", name)).sense(egui::Sense::click()))
                    .clicked()
                {
                    self.load_file(path.clone());
                }
            }
        }

        ui.add_space(16.0);
        ui.label(RichText::new("BOOKMARKS").small().strong().color(theme::TEXT_MUTED));
        ui.add_space(4.0);

        if self.bookmarks.is_empty() {
            ui.label(RichText::new("No bookmarks").italics().color(theme::TEXT_FAINT));
        } else {
            let bookmarks = self.bookmarks.clone();
            for offset in bookmarks {
                if ui
                    .add(
                        egui::Label::new(
                            RichText::new(format!("⭐ 0x{:08X}", offset)).color(theme::ACCENT_GOLD),
                        )
                        .sense(egui::Sense::click()),
                    )
                    .clicked()
                {
                    self.selected_offset = offset;
                }
            }
        }

        ui.add_space(16.0);
        ui.label(RichText::new("SEARCH RESULTS").small().strong().color(theme::TEXT_MUTED));
        ui.add_space(4.0);
        if self.search_results.is_empty() {
            ui.label(RichText::new("No results").italics().color(theme::TEXT_FAINT));
        } else {
            ui.label(
                RichText::new(format!("{} matches", self.search_results.len()))
                    .color(theme::ACCENT_CYAN),
            );
        }
    }

    fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(path) = self.file_path.clone() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string());

                egui::Frame::none()
                    .fill(theme::BG_WIDGET)
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(name).color(theme::TEXT_PRIMARY));
                            ui.add_space(6.0);
                            if ui
                                .add(egui::Label::new(RichText::new("✕").color(theme::TEXT_MUTED))
                                    .sense(egui::Sense::click()))
                                .clicked()
                            {
                                self.close_file();
                            }
                        });
                    });
            } else {
                ui.label(RichText::new("No file open").color(theme::TEXT_FAINT));
            }
        });
        ui.add_space(4.0);
        ui.separator();
    }

    fn render_hex_grid(&mut self, ui: &mut egui::Ui) {
        if self.file_data.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(
                    RichText::new("📂")
                        .size(42.0)
                        .color(theme::TEXT_FAINT),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new("No file loaded")
                        .size(16.0)
                        .color(theme::TEXT_MUTED),
                );
                ui.label(
                    RichText::new("Use File → Open, or the toolbar Open button")
                        .small()
                        .color(theme::TEXT_FAINT),
                );
            });
            return;
        }

        let num_rows = (self.file_data.len() + self.bytes_per_row - 1) / self.bytes_per_row;

        // Header.
        egui::Frame::none()
            .fill(theme::BG_PANEL_ALT)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(6.0, 6.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("OFFSET")
                            .monospace()
                            .strong()
                            .small()
                            .color(theme::TEXT_MUTED),
                    );
                    ui.add_space(24.0);
                    for i in 0..self.bytes_per_row {
                        let label = RichText::new(format!("{:02X}", i))
                            .monospace()
                            .small()
                            .color(theme::TEXT_MUTED);
                        ui.label(label);
                        ui.add_space(2.0);
                    }
                    ui.add_space(16.0);
                    ui.label(
                        RichText::new("ASCII")
                            .monospace()
                            .strong()
                            .small()
                            .color(theme::TEXT_MUTED),
                    );
                });
            });

        ui.add_space(4.0);

        // Data rows.
        egui::ScrollArea::vertical().show(ui, |ui| {
            for row in 0..num_rows {
                let offset = row * self.bytes_per_row;
                if offset >= self.file_data.len() {
                    break;
                }

                let row_selected =
                    offset == self.selected_offset - (self.selected_offset % self.bytes_per_row);

                let frame = egui::Frame::none()
                    .fill(if row_selected {
                        theme::BG_WIDGET_HOVER
                    } else if row % 2 == 0 {
                        theme::BG_PANEL
                    } else {
                        theme::BG_ROW_ALT
                    })
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::symmetric(4.0, 2.0));

                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Offset.
                        let offset_label = RichText::new(format!("{:08X}", offset))
                            .monospace()
                            .color(if row_selected {
                                theme::ACCENT_GOLD
                            } else {
                                theme::ACCENT_CYAN
                            });

                        let offset_response =
                            ui.add(egui::Label::new(offset_label).sense(egui::Sense::click()));
                        if offset_response.clicked() {
                            self.selected_offset = offset;
                        }
                        ui.add_space(20.0);

                        // Hex bytes.
                        for col in 0..self.bytes_per_row {
                            let idx = offset + col;
                            if idx < self.file_data.len() {
                                let byte = self.file_data[idx];
                                let hex_text = format!("{:02X}", byte);
                                let is_selected = self.selected_offset == idx;
                                let is_bookmarked = self.bookmarks.contains(&idx);
                                let is_search_result = self.search_results.contains(&idx);

                                let mut label = RichText::new(hex_text).monospace();

                                if is_selected {
                                    label = label.color(Color32::WHITE);
                                    let response =
                                        ui.add(egui::Label::new(label.clone()).sense(egui::Sense::click()));
                                    let rect = response.rect.expand(1.5);
                                    ui.painter().rect_filled(rect, 4.0, theme::ACCENT);
                                    ui.put(rect, egui::Label::new(label));
                                    if response.clicked() {
                                        self.selected_offset = idx;
                                    }
                                } else if is_search_result {
                                    label = label.color(Color32::BLACK);
                                    let response =
                                        ui.add(egui::Label::new(label.clone()).sense(egui::Sense::click()));
                                    let rect = response.rect.expand(1.5);
                                    ui.painter().rect_filled(rect, 4.0, theme::WARNING);
                                    ui.put(rect, egui::Label::new(label));
                                    if response.clicked() {
                                        self.selected_offset = idx;
                                    }
                                } else if is_bookmarked {
                                    label = label.color(theme::ACCENT_GOLD);
                                    let response = ui.add(egui::Label::new(label).sense(egui::Sense::click()));
                                    if response.clicked() {
                                        self.selected_offset = idx;
                                    }
                                } else {
                                    if byte < 0x20 || byte > 0x7E {
                                        label = label.color(theme::TEXT_FAINT);
                                    } else {
                                        label = label.color(theme::TEXT_PRIMARY);
                                    }
                                    let response = ui.add(egui::Label::new(label).sense(egui::Sense::click()));
                                    if response.clicked() {
                                        self.selected_offset = idx;
                                    }
                                }
                                ui.add_space(2.0);
                            } else {
                                ui.add_space(28.0);
                            }
                        }

                        ui.add_space(16.0);

                        // ASCII representation.
                        let mut ascii_str = String::new();
                        for col in 0..self.bytes_per_row {
                            let idx = offset + col;
                            if idx < self.file_data.len() {
                                let byte = self.file_data[idx];
                                if byte >= 32 && byte <= 126 {
                                    ascii_str.push(byte as char);
                                } else {
                                    ascii_str.push('.');
                                }
                            }
                        }

                        let ascii_label = RichText::new(ascii_str).monospace().color(theme::TEXT_MUTED);
                        ui.label(ascii_label);
                    });
                });
            }
        });

        ui.add_space(4.0);
        ui.separator();
        ui.label(
            RichText::new(format!("Total: {} bytes", self.file_data.len()))
                .small()
                .color(theme::TEXT_FAINT),
        );
    }

    fn render_data_inspector(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("▾").color(theme::TEXT_MUTED));
            ui.label(RichText::new("Data Inspector").strong().size(15.0));
        });
        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        if self.file_data.is_empty() || self.selected_offset >= self.file_data.len() {
            ui.label(RichText::new("No data selected").italics().color(theme::TEXT_FAINT));
            return;
        }

        let byte = self.file_data[self.selected_offset];

        egui::Frame::none()
            .fill(theme::BG_WIDGET)
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::same(8.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("Offset  0x{:08X}", self.selected_offset))
                        .strong()
                        .color(theme::ACCENT_CYAN),
                );
                ui.add_space(2.0);
                egui::Grid::new("byte_basics_grid")
                    .num_columns(2)
                    .spacing([16.0, 2.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Hex").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{:02X}", byte)).monospace());
                        ui.end_row();

                        ui.label(RichText::new("Decimal").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{}", byte)).monospace());
                        ui.end_row();

                        ui.label(RichText::new("Binary").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{:08b}", byte)).monospace());
                        ui.end_row();

                        ui.label(RichText::new("Octal").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{:03o}", byte)).monospace());
                        ui.end_row();
                    });
            });

        ui.add_space(10.0);

        egui::CollapsingHeader::new(RichText::new("Integer (8/16/32/64-bit)").strong())
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("int_grid")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Int8").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{}", byte as i8)).monospace());
                        ui.end_row();
                        ui.label(RichText::new("UInt8").color(theme::TEXT_MUTED));
                        ui.label(RichText::new(format!("{}", byte)).monospace());
                        ui.end_row();

                        if self.selected_offset + 1 < self.file_data.len() {
                            let bytes = [
                                self.file_data[self.selected_offset],
                                self.file_data[self.selected_offset + 1],
                            ];
                            let val16 = match self.endianness {
                                Endianness::Little => u16::from_le_bytes(bytes),
                                Endianness::Big => u16::from_be_bytes(bytes),
                            };
                            ui.label(RichText::new("Int16").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", val16 as i16)).monospace());
                            ui.end_row();
                            ui.label(RichText::new("UInt16").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", val16)).monospace());
                            ui.end_row();
                        }

                        if self.selected_offset + 3 < self.file_data.len() {
                            let bytes: [u8; 4] = [
                                self.file_data[self.selected_offset],
                                self.file_data[self.selected_offset + 1],
                                self.file_data[self.selected_offset + 2],
                                self.file_data[self.selected_offset + 3],
                            ];
                            let int32 = match self.endianness {
                                Endianness::Little => u32::from_le_bytes(bytes),
                                Endianness::Big => u32::from_be_bytes(bytes),
                            };
                            ui.label(RichText::new("Int32").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", int32 as i32)).monospace());
                            ui.end_row();
                            ui.label(RichText::new("UInt32").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", int32)).monospace());
                            ui.end_row();
                        }

                        if self.selected_offset + 7 < self.file_data.len() {
                            let mut bytes = [0u8; 8];
                            bytes.copy_from_slice(
                                &self.file_data[self.selected_offset..self.selected_offset + 8],
                            );
                            let int64 = match self.endianness {
                                Endianness::Little => u64::from_le_bytes(bytes),
                                Endianness::Big => u64::from_be_bytes(bytes),
                            };
                            ui.label(RichText::new("Int64").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", int64 as i64)).monospace());
                            ui.end_row();
                            ui.label(RichText::new("UInt64").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{}", int64)).monospace());
                            ui.end_row();
                        }
                    });
            });

        ui.add_space(6.0);

        egui::CollapsingHeader::new(RichText::new("Float").strong())
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("float_grid")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        if self.selected_offset + 3 < self.file_data.len() {
                            let bytes: [u8; 4] = [
                                self.file_data[self.selected_offset],
                                self.file_data[self.selected_offset + 1],
                                self.file_data[self.selected_offset + 2],
                                self.file_data[self.selected_offset + 3],
                            ];
                            let float_val = match self.endianness {
                                Endianness::Little => f32::from_le_bytes(bytes),
                                Endianness::Big => f32::from_be_bytes(bytes),
                            };
                            ui.label(RichText::new("Float").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{:.6}", float_val)).monospace());
                            ui.end_row();
                        }

                        if self.selected_offset + 7 < self.file_data.len() {
                            let mut bytes = [0u8; 8];
                            bytes.copy_from_slice(
                                &self.file_data[self.selected_offset..self.selected_offset + 8],
                            );
                            let double_val = match self.endianness {
                                Endianness::Little => f64::from_le_bytes(bytes),
                                Endianness::Big => f64::from_be_bytes(bytes),
                            };
                            ui.label(RichText::new("Double").color(theme::TEXT_MUTED));
                            ui.label(RichText::new(format!("{:.6}", double_val)).monospace());
                            ui.end_row();
                        }
                    });
            });

        ui.add_space(6.0);

        egui::CollapsingHeader::new(RichText::new("Binary").strong())
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Endianness").color(theme::TEXT_MUTED));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut is_little = self.endianness == Endianness::Little;
                        if toggle_switch(ui, &mut is_little).changed() {
                            self.endianness = if is_little {
                                Endianness::Little
                            } else {
                                Endianness::Big
                            };
                        }
                    });
                });
                ui.add_space(4.0);
                let endian_text = match self.endianness {
                    Endianness::Little => "Little Endian",
                    Endianness::Big => "Big Endian",
                };
                ui.label(RichText::new(endian_text).color(theme::ACCENT).monospace());
            });

        ui.add_space(6.0);

        egui::CollapsingHeader::new(RichText::new("Character Encoding").strong())
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Encoding").color(theme::TEXT_MUTED));
                    ui.label(
                        RichText::new(self.character_encoding.label())
                            .color(theme::ACCENT)
                            .monospace(),
                    );
                });
                ui.label(format!(
                    "ASCII: {}",
                    if byte >= 32 && byte <= 126 { byte as char } else { '.' }
                ));
            });
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            ui.colored_label(self.status_color, &self.status_message);
            ui.add_space(20.0);
            if let Some(path) = &self.file_path {
                ui.label(
                    RichText::new(format!("File: {}", path.display()))
                        .small()
                        .color(theme::TEXT_FAINT),
                );
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                pill_label(ui, "Line Ending: CRLF", theme::TEXT_MUTED);
                pill_label(ui, format!("Encoding: {}", self.character_encoding.label()), theme::TEXT_MUTED);
                pill_label(ui, "Mode: Overwrite", theme::TEXT_MUTED);
                pill_label(ui, format!("Size: {} bytes", self.file_data.len()), theme::TEXT_MUTED);
                pill_label(ui, "Selected: 1 byte", theme::TEXT_MUTED);
                pill_label(
                    ui,
                    format!("Offset: 0x{:08X}", self.selected_offset),
                    theme::ACCENT_CYAN,
                );
            });
        });
    }

    fn render_search_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_search_dialog;
        egui::Window::new("🔍 Search")
            .open(&mut open)
            .resizable(false)
            .default_pos([120.0, 120.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(theme::BG_PANEL)
                    .rounding(Rounding::same(10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    let response = ui.text_edit_singleline(&mut self.search_query);
                    if response.changed() {
                        self.perform_search();
                    }
                    if ui.button("Search").clicked() {
                        self.perform_search();
                    }
                });

                if !self.search_results.is_empty() {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("Found {} results", self.search_results.len()))
                            .color(theme::ACCENT_CYAN),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("◀ Previous").clicked() {
                            self.navigate_search(-1);
                        }
                        if ui.button("Next ▶").clicked() {
                            self.navigate_search(1);
                        }
                        ui.label(format!(
                            "{}/{}",
                            self.current_search_index + 1,
                            self.search_results.len()
                        ));
                    });
                }
            });
        self.show_search_dialog = open;
    }

    fn render_go_to_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_go_to_dialog;
        let mut close_now = false;
        egui::Window::new("📍 Go To Offset")
            .open(&mut open)
            .resizable(false)
            .default_pos([120.0, 170.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(theme::BG_PANEL)
                    .rounding(Rounding::same(10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Offset:");
                    let response = ui.text_edit_singleline(&mut self.go_to_offset);

                    let enter_pressed = response.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if ui.button("Go").clicked() || enter_pressed {
                        let offset_str = self.go_to_offset.trim().trim_start_matches("0x");
                        if let Ok(offset) = usize::from_str_radix(offset_str, 16) {
                            if offset < self.file_data.len() {
                                self.selected_offset = offset;
                                close_now = true;
                            }
                        }
                    }
                });
            });
        self.show_go_to_dialog = open && !close_now;
    }

    fn render_edit_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_edit_dialog;
        egui::Window::new("✏ Edit Bytes")
            .open(&mut open)
            .resizable(false)
            .default_pos([120.0, 220.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(theme::BG_PANEL)
                    .rounding(Rounding::same(10.0)),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("Edit byte at specific position").color(theme::TEXT_MUTED),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Line (1-indexed):");
                    ui.text_edit_singleline(&mut self.edit_line);
                });
                ui.horizontal(|ui| {
                    ui.label("Offset (1-indexed):");
                    ui.text_edit_singleline(&mut self.edit_offset);
                });
                ui.horizontal(|ui| {
                    ui.label("New data (hex):");
                    ui.text_edit_singleline(&mut self.edit_data);
                });

                ui.add_space(6.0);
                if ui.button("Apply").clicked() {
                    if let Ok(line) = self.edit_line.trim().parse::<usize>() {
                        if let Ok(offset) = self.edit_offset.trim().parse::<usize>() {
                            let new_bytes = crate::parser::parse_hex_bytes(&self.edit_data);
                            if let Some(new_data) = new_bytes {
                                if let Some(path) = &self.file_path {
                                    use crate::editor::write_bytes_to_file;
                                    match write_bytes_to_file(
                                        &path.to_string_lossy(),
                                        line,
                                        offset,
                                        new_data.len(),
                                        &new_data,
                                    ) {
                                        Ok(_) => {
                                            if let Ok(data) = fs::read(path) {
                                                self.file_data = data;
                                                self.status_message =
                                                    "Bytes updated successfully".to_string();
                                                self.status_color = theme::SUCCESS;
                                                self.show_edit_dialog = false;
                                            }
                                        }
                                        Err(e) => {
                                            self.status_message = format!("Error: {}", e);
                                            self.status_color = theme::DANGER;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        self.show_edit_dialog = open && self.show_edit_dialog;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.dark_mode);

        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::none()
                    .fill(theme::BG_PANEL)
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0))
                    .stroke(Stroke::new(1.0, theme::BORDER)),
            )
            .show(ctx, |ui| {
                self.render_menu_bar(ui);
            });

        egui::TopBottomPanel::top("toolbar")
            .frame(
                egui::Frame::none()
                    .fill(theme::BG_PANEL_ALT)
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                    .stroke(Stroke::new(1.0, theme::BORDER)),
            )
            .show(ctx, |ui| {
                self.render_toolbar(ui);
            });

        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::none()
                    .fill(theme::BG_PANEL)
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                    .stroke(Stroke::new(1.0, theme::BORDER)),
            )
            .show(ctx, |ui| {
                self.render_status_bar(ui);
            });

        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(210.0)
            .width_range(160.0..=320.0)
            .frame(
                egui::Frame::none()
                    .fill(theme::BG_PANEL)
                    .inner_margin(egui::Margin::symmetric(10.0, 10.0))
                    .stroke(Stroke::new(1.0, theme::BORDER)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_sidebar(ui);
                    });
            });

        if self.show_data_inspector {
            egui::SidePanel::right("inspector")
                .resizable(true)
                .default_width(260.0)
                .width_range(200.0..=400.0)
                .frame(
                    egui::Frame::none()
                        .fill(theme::BG_PANEL)
                        .inner_margin(egui::Margin::symmetric(10.0, 10.0))
                        .stroke(Stroke::new(1.0, theme::BORDER)),
                )
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            self.render_data_inspector(ui);
                        });
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(theme::BG_APP)
                    .inner_margin(egui::Margin::symmetric(10.0, 8.0)),
            )
            .show(ctx, |ui| {
                self.render_tab_bar(ui);
                self.render_hex_grid(ui);
            });

        // Dialogs.
        if self.show_search_dialog {
            self.render_search_dialog(ctx);
        }
        if self.show_go_to_dialog {
            self.render_go_to_dialog(ctx);
        }
        if self.show_edit_dialog {
            self.render_edit_dialog(ctx);
        }
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("AtomEX - Hex Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "AtomEX",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
