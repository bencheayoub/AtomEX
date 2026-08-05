// src/gui.rs
use eframe::egui;
use egui::{Color32, RichText};
use std::fs;
use std::path::PathBuf;

#[derive(Default, Clone)]
struct EditOperation {
    line: usize,
    offset: usize,
    old_data: Vec<u8>,
    new_data: Vec<u8>,
    timestamp: u64,
}

#[derive(Default)]
pub struct MyApp {
    // File data
    file_path: Option<PathBuf>,
    file_data: Vec<u8>,
    original_data: Vec<u8>,
    
    // UI state
    selected_offset: usize,
    bytes_per_row: usize,
    show_data_inspector: bool,
    show_search_dialog: bool,
    show_go_to_dialog: bool,
    show_edit_dialog: bool,
    
    // Search
    search_query: String,
    search_results: Vec<usize>,
    current_search_index: usize,
    
    // Edit dialog
    edit_line: String,
    edit_offset: String,
    edit_data: String,
    
    // Go to
    go_to_offset: String,
    
    // Settings
    character_encoding: Encoding,
    endianness: Endianness,
    
    // Undo/Redo
    undo_stack: Vec<EditOperation>,
    redo_stack: Vec<EditOperation>,
    operation_counter: u64,
    
    // Recent files
    recent_files: Vec<PathBuf>,
    
    // Status
    status_message: String,
    status_color: Color32,
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

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up custom styling
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(2.0, 2.0);
        cc.egui_ctx.set_style(style);
        
        let mut app = MyApp::default();
        app.bytes_per_row = 16;
        app.show_data_inspector = true;
        app.status_message = "Ready".to_string();
        app.status_color = Color32::GRAY;
        
        app
    }
    
    fn load_file(&mut self, path: PathBuf) {
        match fs::read(&path) {
            Ok(data) => {
                // Store the path for display after moving
                let path_display = path.display().to_string();
                self.file_path = Some(path);
                self.file_data = data.clone();
                self.original_data = data;
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.selected_offset = 0;
                
                // Add to recent files (clone before moving)
                let path_clone = PathBuf::from(&path_display);
                if !self.recent_files.contains(&path_clone) {
                    self.recent_files.insert(0, path_clone);
                    if self.recent_files.len() > 10 {
                        self.recent_files.pop();
                    }
                }
                
                self.status_message = format!("Loaded: {}", path_display);
                self.status_color = Color32::GREEN;
            }
            Err(e) => {
                self.status_message = format!("Error loading file: {}", e);
                self.status_color = Color32::RED;
            }
        }
    }
    
    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            match fs::write(path, &self.file_data) {
                Ok(_) => {
                    self.original_data = self.file_data.clone();
                    self.status_message = format!("Saved: {}", path.display());
                    self.status_color = Color32::GREEN;
                }
                Err(e) => {
                    self.status_message = format!("Error saving: {}", e);
                    self.status_color = Color32::RED;
                }
            }
        }
    }
    
    fn perform_search(&mut self) {
        self.search_results.clear();
        if self.search_query.is_empty() || self.file_data.is_empty() {
            return;
        }
        
        // Try to parse as hex first
        let query_bytes = if self.search_query.contains(' ') {
            // Try hex bytes
            self.search_query
                .split_whitespace()
                .filter_map(|s| u8::from_str_radix(s, 16).ok())
                .collect::<Vec<u8>>()
        } else {
            // Try as ASCII or hex string
            if self.search_query.starts_with("0x") || self.search_query.chars().all(|c| c.is_ascii_hexdigit()) {
                let hex = self.search_query.trim_start_matches("0x");
                if hex.len() % 2 == 0 {
                    (0..hex.len())
                        .step_by(2)
                        .filter_map(|i| u8::from_str_radix(&hex[i..i+2], 16).ok())
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
            self.status_color = Color32::BLUE;
        } else {
            self.status_message = "No results found".to_string();
            self.status_color = Color32::YELLOW;
        }
    }
    
    fn navigate_search(&mut self, direction: i32) {
        if self.search_results.is_empty() {
            return;
        }
        
        let new_index = (self.current_search_index as i32 + direction).rem_euclid(self.search_results.len() as i32) as usize;
        self.current_search_index = new_index;
        self.selected_offset = self.search_results[new_index];
    }
    
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // File menu
            ui.menu_button("File", |ui| {
                if ui.button("📂 Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.load_file(path);
                    }
                    ui.close_menu();
                }
                if ui.button("💾 Save").clicked() {
                    self.save_file();
                    ui.close_menu();
                }
                if ui.button("Save As").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        let _ = fs::write(path, &self.file_data);
                    }
                    ui.close_menu();
                }
                ui.separator();
                
                // Recent files
                if !self.recent_files.is_empty() {
                    ui.label("Recent Files");
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
            
            // Edit menu
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() { ui.close_menu(); }
                if ui.button("Redo").clicked() { ui.close_menu(); }
                ui.separator();
                if ui.button("Cut").clicked() { ui.close_menu(); }
                if ui.button("Copy").clicked() { ui.close_menu(); }
                if ui.button("Paste").clicked() { ui.close_menu(); }
            });
            
            // View menu
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut self.show_data_inspector, "Data Inspector");
                ui.separator();
                if ui.button("Refresh").clicked() { ui.close_menu(); }
            });
            
            // Search menu
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
            
            // Analyze menu
            ui.menu_button("Analyze", |ui| {
                if ui.button("Find Patterns").clicked() { ui.close_menu(); }
                if ui.button("Check Integrity").clicked() { ui.close_menu(); }
            });
            
            // Settings menu
            ui.menu_button("Settings", |ui| {
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
            
            // Theme menu
            ui.menu_button("Theme", |ui| {
                if ui.button("Dark").clicked() { ui.close_menu(); }
                if ui.button("Light").clicked() { ui.close_menu(); }
            });
            
            // Help menu
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() { 
                    ui.close_menu();
                }
            });
        });
    }
    
    fn render_hex_grid(&mut self, ui: &mut egui::Ui) {
        if self.file_data.is_empty() {
            ui.label(RichText::new("No file loaded. Use File -> Open to load a file.")
                .color(Color32::GRAY)
                .heading());
            return;
        }
        
        let num_rows = (self.file_data.len() + self.bytes_per_row - 1) / self.bytes_per_row;
        
        // Header
        ui.horizontal(|ui| {
            ui.label(RichText::new("Offset").monospace().strong());
            ui.add_space(30.0);
            for i in 0..self.bytes_per_row {
                let label = RichText::new(format!("{:02X}", i)).monospace().small();
                ui.label(label);
                ui.add_space(2.0);
            }
            ui.add_space(20.0);
            ui.label(RichText::new("ASCII").monospace().strong());
        });
        
        ui.separator();
        
        // Data rows
        egui::ScrollArea::vertical().show(ui, |ui| {
            for row in 0..num_rows {
                let offset = row * self.bytes_per_row;
                if offset >= self.file_data.len() {
                    break;
                }
                
                ui.horizontal(|ui| {
                    // Offset
                    let offset_label = RichText::new(format!("{:08X}", offset))
                        .monospace()
                        .color(if offset == self.selected_offset - (self.selected_offset % self.bytes_per_row) {
                            Color32::YELLOW
                        } else {
                            Color32::from_rgb(100, 200, 200)
                        });
                    
                    let offset_response = ui.add(egui::Label::new(offset_label).sense(egui::Sense::click()));
                    if offset_response.clicked() {
                        self.selected_offset = offset;
                    }
                    ui.add_space(20.0);
                    
                    // Hex bytes
                    for col in 0..self.bytes_per_row {
                        let idx = offset + col;
                        if idx < self.file_data.len() {
                            let byte = self.file_data[idx];
                            let hex_text = format!("{:02X}", byte);
                            let is_selected = self.selected_offset == idx;
                            let is_search_result = self.search_results.contains(&idx);
                            
                            // Create label with appropriate styling
                            let mut label = RichText::new(hex_text).monospace();
                            
                            if is_selected {
                                label = label.color(Color32::WHITE);
                                let response = ui.add(egui::Label::new(label.clone()).sense(egui::Sense::click()));
                                // Draw background manually
                                let rect = response.rect;
                                ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(0, 120, 255));
                                // Re-add the label on top
                                ui.put(rect, egui::Label::new(label));
                                if response.clicked() {
                                    self.selected_offset = idx;
                                }
                            } else if is_search_result {
                                label = label.color(Color32::BLACK);
                                let response = ui.add(egui::Label::new(label.clone()).sense(egui::Sense::click()));
                                let rect = response.rect;
                                ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(255, 255, 100));
                                ui.put(rect, egui::Label::new(label));
                                if response.clicked() {
                                    self.selected_offset = idx;
                                }
                            } else {
                                // Color based on byte value
                                if byte < 0x20 || byte > 0x7E {
                                    label = label.color(Color32::from_rgb(150, 150, 150));
                                } else {
                                    label = label.color(Color32::from_rgb(200, 200, 200));
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
                    
                    ui.add_space(20.0);
                    
                    // ASCII representation
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
                    
                    let ascii_label = RichText::new(ascii_str).monospace();
                    ui.label(ascii_label);
                });
            }
        });
        
        // Show total bytes
        ui.separator();
        ui.label(RichText::new(format!("Total: {} bytes", self.file_data.len()))
            .small()
            .color(Color32::GRAY));
    }
    
    fn render_data_inspector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Inspector");
        ui.separator();
        
        if self.file_data.is_empty() || self.selected_offset >= self.file_data.len() {
            ui.colored_label(Color32::GRAY, "No data selected");
            return;
        }
        
        let byte = self.file_data[self.selected_offset];
        
        ui.label(RichText::new(format!("Offset: 0x{:08X}", self.selected_offset)).strong());
        ui.label(format!("Hex: {:02X}", byte));
        ui.label(format!("Decimal: {}", byte));
        ui.label(format!("Binary: {:08b}", byte));
        ui.label(format!("Octal: {:03o}", byte));
        
        ui.separator();
        ui.label(RichText::new("Integer Interpretations").strong());
        
        // Int8
        ui.label(format!("Int8: {}", byte as i8));
        ui.label(format!("UInt8: {}", byte));
        
        if self.selected_offset + 1 < self.file_data.len() {
            let bytes = [self.file_data[self.selected_offset], self.file_data[self.selected_offset + 1]];
            let val16 = match self.endianness {
                Endianness::Little => u16::from_le_bytes(bytes),
                Endianness::Big => u16::from_be_bytes(bytes),
            };
            ui.label(format!("Int16: {}", val16 as i16));
            ui.label(format!("UInt16: {}", val16));
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
            ui.label(format!("Int32: {}", int32 as i32));
            ui.label(format!("UInt32: {}", int32));
            
            let float_val = match self.endianness {
                Endianness::Little => f32::from_le_bytes(bytes),
                Endianness::Big => f32::from_be_bytes(bytes),
            };
            ui.label(format!("Float: {:.6}", float_val));
        }
        
        if self.selected_offset + 7 < self.file_data.len() {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&self.file_data[self.selected_offset..self.selected_offset + 8]);
            
            let int64 = match self.endianness {
                Endianness::Little => u64::from_le_bytes(bytes),
                Endianness::Big => u64::from_be_bytes(bytes),
            };
            ui.label(format!("Int64: {}", int64 as i64));
            ui.label(format!("UInt64: {}", int64));
            
            let double_val = match self.endianness {
                Endianness::Little => f64::from_le_bytes(bytes),
                Endianness::Big => f64::from_be_bytes(bytes),
            };
            ui.label(format!("Double: {:.6}", double_val));
        }
        
        ui.separator();
        ui.label(RichText::new("Character Interpretation").strong());
        ui.label(format!("ASCII: {}", if byte >= 32 && byte <= 126 { byte as char } else { '.' }));
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set up the frame
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.render_menu_bar(ui);
        });
        
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(self.status_color, &self.status_message);
                ui.add_space(20.0);
                if let Some(path) = &self.file_path {
                    ui.label(RichText::new(format!("File: {}", path.display())).small());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("Offset: 0x{:08X}", self.selected_offset)).small());
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("Size: {} bytes", self.file_data.len())).small());
                });
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Split into hex editor and data inspector
            ui.horizontal(|ui| {
                // Left: Hex editor
                ui.vertical(|ui| {
                    // Toolbar
                    ui.horizontal(|ui| {
                        if ui.button("📂 Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.load_file(path);
                            }
                        }
                        if ui.button("💾 Save").clicked() {
                            self.save_file();
                        }
                        ui.add_space(10.0);
                        if ui.button("🔍 Search").clicked() {
                            self.show_search_dialog = !self.show_search_dialog;
                        }
                        if ui.button("📍 Go To").clicked() {
                            self.show_go_to_dialog = !self.show_go_to_dialog;
                        }
                        if ui.button("✏️ Edit").clicked() {
                            self.show_edit_dialog = !self.show_edit_dialog;
                        }
                    });
                    
                    ui.separator();
                    
                    // Hex grid
                    self.render_hex_grid(ui);
                });
                
                // Right: Data inspector
                if self.show_data_inspector {
                    ui.separator();
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                self.render_data_inspector(ui);
                            });
                    });
                }
            });
        });
        
        // Dialogs
        if self.show_search_dialog {
            egui::Window::new("Search")
                .resizable(false)
                .default_pos([100.0, 100.0])
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
                        ui.label(format!("Found {} results", self.search_results.len()));
                        ui.horizontal(|ui| {
                            if ui.button("◀ Previous").clicked() {
                                self.navigate_search(-1);
                            }
                            if ui.button("Next ▶").clicked() {
                                self.navigate_search(1);
                            }
                            ui.label(format!("{}/{}", self.current_search_index + 1, self.search_results.len()));
                        });
                    }
                    
                    if ui.button("Close").clicked() {
                        self.show_search_dialog = false;
                    }
                });
        }
        
        if self.show_go_to_dialog {
            egui::Window::new("Go To Offset")
                .resizable(false)
                .default_pos([100.0, 150.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Offset:");
                        let response = ui.text_edit_singleline(&mut self.go_to_offset);
                        if ui.button("Go").clicked() || response.changed() {
                            let offset_str = self.go_to_offset.trim().trim_start_matches("0x");
                            if let Ok(offset) = usize::from_str_radix(offset_str, 16) {
                                if offset < self.file_data.len() {
                                    self.selected_offset = offset;
                                    self.show_go_to_dialog = false;
                                }
                            }
                        }
                    });
                    
                    if ui.button("Close").clicked() {
                        self.show_go_to_dialog = false;
                    }
                });
        }
        
        if self.show_edit_dialog {
            egui::Window::new("Edit Bytes")
                .resizable(false)
                .default_pos([100.0, 200.0])
                .show(ctx, |ui| {
                    ui.label("Edit byte at specific position");
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
                                            &new_data
                                        ) {
                                            Ok(_) => {
                                                // Reload the file
                                                if let Ok(data) = fs::read(path) {
                                                    self.file_data = data;
                                                    self.status_message = "Bytes updated successfully".to_string();
                                                    self.status_color = Color32::GREEN;
                                                    self.show_edit_dialog = false;
                                                }
                                            }
                                            Err(e) => {
                                                self.status_message = format!("Error: {}", e);
                                                self.status_color = Color32::RED;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if ui.button("Close").clicked() {
                        self.show_edit_dialog = false;
                    }
                });
        }
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("AtomEX - Hex Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "AtomEX",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}