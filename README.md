# AtomEX

## 🚧 Project Status: Under Development

AtomEX is currently an active work-in-progress project. It is not finished yet, and many planned features are still being implemented. The current version focuses on building the core foundations, experimenting with Rust systems programming concepts, and gradually evolving toward a complete graphical hex editor.


A modern, high-performance graphical hex editor built in Rust.

AtomEX lets you view, navigate, and edit binary data directly — displaying files
as raw bytes in both hexadecimal and ASCII form. It's built for programmers,
reverse engineers, and anyone who needs to work at the byte level with
executables, firmware images, and other binary formats.

This project is developed as a hands-on learning exercise in systems
programming with Rust: file I/O, memory management, binary data
representation, and GUI development from the ground up.

## Status

🚧 Early development — currently a CLI prototype for core file-reading logic.
GUI work has not started yet.

## Features

### Planned

- **Binary visualization**
  - Hex + ASCII side-by-side view
  - Efficient navigation through large files
  - Offset tracking and "go to address"

- **Editing**
  - Overwrite, insert, and delete bytes
  - Undo / redo
  - Save with file integrity preserved

- **GUI**
  - Cross-platform, responsive interface
  - Workflow inspired by professional reverse-engineering tools

- **Analysis tools**
  - Byte pattern / string search
  - Data inspector (interpret bytes as int8/16/32/float, endianness toggle)
  - Basic file-format highlighting (e.g. PE/ELF headers)

### Implemented

- Read a file path from user input
- Load file contents into memory
- CLI menu skeleton (read / write / exit)

## Tech Stack

- **Language:** Rust — memory safety, performance, and a strong fit for
  low-level binary manipulation
- **GUI framework:** TBD (currently evaluating `egui` for its immediate-mode
  rendering, which suits a scrollable byte-grid view)

## Architecture Notes

Large binary files (firmware dumps, disk images, executables) won't fit
comfortably as a single in-memory buffer forever. The long-term plan is to
move from a naive `Vec<u8>` file buffer to a **piece table** structure —
keeping the original file untouched on disk/mmap and recording edits as an
overlay — to support efficient insert/delete and large-file editing without
rewriting the whole file on every change.

## Goals

This project exists as much for the learning as for the tool itself. Areas of
focus:

- File I/O and binary manipulation in Rust
- Memory management and ownership in a systems context
- Data representation at the byte level
- GUI development for data-dense interfaces
- Reverse-engineering workflows and tooling

## Getting Started

```bash
git clone <repo-url>
cd atomex
cargo run
```

> Requires a recent stable Rust toolchain (`rustup` recommended).

## Roadmap

1. CLI hex dumper (hex + ASCII output, no GUI)
2. GUI framework fundamentals (isolated prototypes)
3. Static hex/ASCII grid view (read-only)
4. Byte selection and keyboard navigation
5. In-memory editing (small files)
6. Undo / redo
7. Insert / delete support
8. Piece table for efficient editing
9. Large file support (memory-mapped I/O)
10. Reverse-engineering features (search, data inspector, format highlighting)
11. Polish (themes, shortcuts, recent files)