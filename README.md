# Papyrender

A fast, dependency-light HTML-to-PDF rendering engine written in Rust.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Status: early development](https://img.shields.io/badge/status-early%20development-yellow.svg)](#status)

## Overview

Papyrender turns HTML documents into PDF files through a small, explicit
pipeline:

```
HTML source ─▶ parse ─▶ style ─▶ layout ─▶ pdf ─▶ PDF bytes
```

- **parse** — turns HTML text into a DOM tree ([`html5ever`](https://github.com/servo/html5ever))
- **style** — resolves CSS rules against the DOM into a styled tree
- **layout** — computes box positions and sizes with a flexbox/grid engine ([`taffy`](https://github.com/DioxusLabs/taffy))
- **pdf** — writes the laid-out boxes to PDF

Each stage is a separate module in [`papyrender-core`](core), so it can be
tested, benchmarked, and swapped out independently.

## Status

Papyrender is in **early scaffolding**. The workspace, the `parse` stage, and
the pipeline wiring are in place; `style`, `layout`, and `pdf` are currently
placeholders. Expect the API to change without notice until a `0.x` release
is tagged. See the [Roadmap](#roadmap) for what's next.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- On Windows with the MSVC toolchain, the "Desktop development with C++"
  workload from Visual Studio Build Tools (needed to link native crates)

### Build & test

```sh
cargo build --workspace
cargo test --workspace
```

### Run the CLI

```sh
cargo run -p papyrender-cli -- path/to/input.html
```

## Project Structure

```
Papyrender/
├── core/               # papyrender-core: the parse → style → layout → pdf pipeline
│   └── src/
│       ├── parse.rs    # HTML → DOM (html5ever)
│       ├── style.rs    # DOM → styled tree
│       ├── layout.rs   # styled tree → box tree (taffy)
│       ├── pdf.rs       # box tree → PDF bytes
│       └── lib.rs      # pipeline wiring
└── cli/                # papyrender-cli: command-line entry point
    └── src/main.rs
```

## Roadmap

- [ ] CSS parsing and cascade/specificity resolution in `style`
- [ ] Box tree construction and text layout in `layout`
- [ ] PDF document writer in `pdf` (fonts, images, pagination)
- [ ] CLI options for output path, page size, and margins
- [ ] CI (build, test, lint) on Linux/macOS/Windows

## Contributing

Issues and pull requests are welcome. The project is still taking shape, so
if you're planning a larger change, open an issue first to discuss direction
and avoid duplicated work.

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
