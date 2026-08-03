// imports from rust.

use eframe::egui;
use egui::{Color32, RichText};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

// import from crate.
use crate::hex::{file_to_bytes, hex_dump};
use crate::view::{show_lines, show_line};
use crate::editor::write_bytes_to_file;

#[derive(default, clone)]
struct EditOptions {
    line: usize,
    offset: usize,
    old_bytes: Vec<u8>,
    new_bytes: Vec<u8>,
    timestamp: u64,
}

