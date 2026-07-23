# AtomEX

> **A modern, high-performance hex editor written in Rust.**

AtomEX is an open-source project focused on building a fast, reliable, and modern hexadecimal editor for binary file analysis and editing. The project is being developed from scratch as a way to deeply explore Rust systems programming while creating a practical reverse-engineering tool.

It allows binary files to be viewed and edited at the byte level through hexadecimal and ASCII representations, with a long-term goal of providing a professional graphical interface comparable to established hex editors.

---

# 🚧 Project Status

AtomEX is currently under active development.

The project has successfully completed its initial command-line implementation and is now entering the **GUI development phase**. Core file manipulation features have been implemented, providing a solid foundation before moving to a graphical interface.

Although many advanced capabilities are still planned, the project is already capable of reading, navigating, displaying, and modifying binary files directly.

---

# Current Features

### File Handling

* Open binary files
* Load entire files into memory
* Reload modified files automatically
* Error handling for invalid paths and I/O failures

### Hex Viewer

* Complete hexadecimal dump
* Hexadecimal + ASCII representation
* Configurable line display
* Display individual lines by offset
* Automatic formatting with offsets

Example:

```text
00000000  4d 5a 90 00 03 00 00 00 04 00 00 00 ff ff 00 00  MZ..............
00000010  b8 00 00 00 00 00 00 00 40 00 00 00 00 00 00 00  ........@.......
```

### Editing

* Edit arbitrary bytes
* Write hexadecimal values directly
* Automatic validation of hexadecimal input
* Bounds checking
* Prevent out-of-range writes
* Automatic file reload after modification

### Navigation

* View specific offsets
* View multiple consecutive lines
* Offset parsing using hexadecimal notation (`0x80`, `100`, etc.)

### Utility

* Hexadecimal parser
* ASCII conversion
* Hexadecimal formatting helpers
* Clean CLI menu system

---

# Planned Features

## Graphical User Interface

* Modern desktop interface
* Dockable panels
* Multiple opened files
* Dark and light themes
* Responsive layout
* Keyboard shortcuts
* Context menus

---

## Hex Editing

* Insert bytes
* Delete bytes
* Overwrite mode
* Undo / Redo
* Clipboard support
* Find & Replace

---

## Navigation

* Go to Offset
* Jump history
* Address bookmarks
* Byte selection
* Keyboard navigation

---

## Search

* Search hexadecimal values
* Search ASCII strings
* Unicode search
* Pattern search
* Highlight results

---

## Data Inspector

Interpret selected bytes as:

* int8 / int16 / int32 / int64
* uint8 / uint16 / uint32 / uint64
* float32
* float64
* ASCII
* UTF-8
* UTF-16

with little-endian and big-endian support.

---

## Reverse Engineering Features

* PE parsing
* ELF parsing
* Header highlighting
* Entropy visualization
* Strings extraction
* Structure viewer

---

## Large File Support

Future versions will replace the current in-memory buffer with more efficient data structures.

Planned improvements include:

* Piece Table
* Memory-Mapped Files (mmap)
* Lazy loading
* Efficient editing of multi-gigabyte files

---

# Technology

| Component         | Technology         |
| ----------------- | ------------------ |
| Language          | Rust               |
| Current Interface | Command-Line (CLI) |
| Planned GUI       | egui               |
| File Handling     | std::fs            |
| Build System      | Cargo              |

---

# Project Goals

AtomEX is both a practical application and a learning project designed to deepen understanding of:

* Rust ownership and borrowing
* Systems programming
* Binary file manipulation
* Memory management
* File I/O
* Data structures
* GUI application development
* Reverse engineering fundamentals

---

# Roadmap

* [x] Project setup
* [x] File loading
* [x] Hexadecimal dump
* [x] ASCII visualization
* [x] Offset display
* [x] Display specific lines
* [x] Display multiple lines
* [x] Hexadecimal parser
* [x] Binary editing
* [x] Write changes back to file
* [x] Input validation
* [ ] GUI foundation
* [ ] Hex grid widget
* [ ] Byte selection
* [ ] Keyboard navigation
* [ ] Undo / Redo
* [ ] Search
* [ ] Data inspector
* [ ] Large file support
* [ ] Piece Table implementation
* [ ] Reverse engineering tools

---

# Getting Started

```bash
git clone https://github.com/bencheayoub/AtomEX.git

cd AtomEX

cargo run
```

Requirements:

* Latest stable Rust
* Cargo

---

# Vision

The long-term objective of AtomEX is to become a professional, cross-platform hexadecimal editor capable of handling very large binary files efficiently while remaining lightweight, responsive, and entirely written in safe Rust wherever practical.

The current CLI implementation establishes the project's core editing engine, while upcoming development will focus on building a polished graphical interface and expanding reverse-engineering capabilities.
