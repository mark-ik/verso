# Verso

Verso is an experimental browser built on [Servo](https://servo.org/), using Rust and WebRender for rendering.

## Current State

Verso is currently a **tab-based browser** with support for:
- Multiple tabs and windows via Winit event loop
- WebRender-based rendering with Servo integration
- Download management
- Configuration system
- Clipboard support (via arboard)
- Keyboard and touch input handling

**Planned**: Migration to a force-directed graph canvas interface for spatial browsing (see [design_docs/](design_docs/) for research and specifications).

## Quick Start

```powershell
git clone https://github.com/markik/verso
cd verso
cargo build --release
./target/release/verso
```

## Requirements

- Rust (see [rust-toolchain.toml](rust-toolchain.toml))
- Platform tooling for Servo builds
  - **Windows**: MozillaBuild (includes LLVM, clang, etc.) — see [design_docs/SERVO_MIGRATION_SUMMARY.md](design_docs/SERVO_MIGRATION_SUMMARY.md)
  - **Linux/macOS**: Standard development toolchain
  - **Alternative**: Nix shell (`nix-shell` using [shell.nix](shell.nix))

## Architecture

### Core Components

- **[src/main.rs](src/main.rs)**: Winit-based event loop with ApplicationHandler
- **[src/verso.rs](src/verso.rs)**: Main Verso struct integrating Servo constellation, compositor, and webview pool
- **[src/compositor.rs](src/compositor.rs)**: Rendering coordination with WebRender and display lists
- **[src/window.rs](src/window.rs)**: Window management and event handling
- **[src/tab.rs](src/tab.rs)**: Tab data structures
- **[src/webview/](src/webview/)**: WebView embedding and context menu handling
  - [context_menu.rs](src/webview/context_menu.rs): Right-click menu
  - [webview.rs](src/webview/webview.rs): WebView lifecycle management
  - [prompt.rs](src/webview/prompt.rs): Alert/prompt dialogs
- **[src/download.rs](src/download.rs)**: Download manager
- **[src/storage.rs](src/storage.rs)**: Persistence layer
- **[src/config.rs](src/config.rs)**: Configuration management
- **[src/keyboard.rs](src/keyboard.rs)**: Keyboard input handling
- **[src/touch.rs](src/touch.rs)**: Touch input handling
- **[src/rendering.rs](src/rendering.rs)**: Rendering utilities
- **[src/errors.rs](src/errors.rs)**: Error types

### Crates

- **verso** (library): Builder pattern and public API ([verso/src/main.rs](verso/src/main.rs) demonstrates usage)
- **versoview_messages**: IPC message types for webview communication
- **versoview_build**: Build support utilities

### Dependencies

Key external dependencies:
- **Servo**: constellation (tab/pipeline management), compositor, script, layout, canvas, webrender
- **Winit**: Window creation and event loop
- **crossbeam**: Channel-based concurrency
- **ipc-channel**: Inter-process communication
- **arboard**: Clipboard access
- **serde**: Serialization/deserialization

## Building

### Standard Build
```powershell
cargo build --release
```

### With Nix (Linux/macOS)
```bash
nix-shell
cargo build --release
```

### Windows with MozillaBuild
Follow Servo's official setup, then:
```powershell
cargo build --release
```

## Design Documents

Research, specifications, and future roadmap:
- [design_docs/GRAPH_INTERFACE.md](design_docs/GRAPH_INTERFACE.md) — Interaction model for planned graph canvas
- [design_docs/GRAPH_BROWSER_MIGRATION.md](design_docs/GRAPH_BROWSER_MIGRATION.md) — Migration plan from tabs to graph
- [design_docs/PROJECT_PHILOSOPHY.md](design_docs/PROJECT_PHILOSOPHY.md) — Vision and design principles
- [design_docs/VERSE.md](design_docs/VERSE.md) — Phase 3+ tokenization and P2P research
- [design_docs/](design_docs/) — Full archive of research and specifications

## Contributing

See [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md) and [.github/CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md).

## License

Dual-licensed: MIT or Apache-2.0

## References

- [Servo Browser Engine](https://servo.org/)
- [WebRender](https://github.com/servo/webrender)
- [Winit](https://github.com/rust-windowing/winit)
