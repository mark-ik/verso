# Verso

Verso is a local-first spatial browser that represents webpages as nodes in a force-directed graph. It is an experimental research tool focused on sense-making and exploratory workflows.

## Project Status

**Current Phase:** Design and planning  
**Implementation Status:** This is a fork of Servo implementing standard tabbed browsing. The spatial graph browser described in this documentation is planned but not yet implemented.

The architecture, features, and roadmap described below represent design goals and future development plans.

Canonical documentation

- `GRAPH_INTERFACE.md` — Interaction model, physics presets, renderer/camera, and implementation notes.
- `VERSE.md` — Phase 3 research: tokenization model, peer roles, storage economics, and governance.
- `archive_docs/` — Agglomerized design notes (archival copies).

Quick start

```powershell
git clone https://github.com/markik/verso
cd verso
cargo build --release
./target/release/verso
```

Requirements

- Rust (see `rust-toolchain.toml`).
- Platform tooling for Servo builds: see `archive_docs/SERVO_MIGRATION_SUMMARY.md` if you plan to build Servo on Windows.

Architecture

`verso-graph-core` is a standalone library containing graph model, physics, rendering primitives, camera, serialization, and pluggable traits. The `verso` application integrates a UI backend (egui for MVP) and a browser engine (Servo for MVP).

Modular design enables **embedding**—integrating the graph canvas into different host applications or contexts:

- **UI backend trait**: Switch between egui, Xilem+Vello, or GPUI. Allows the graph canvas to render in different UI frameworks without changing core logic.
- **Browser engine trait**: Servo (MVP), Tao+Wry (Chromium backend), or other implementations. Each handles webview rendering independently.
- **Data format**: Graph metadata stored as JSON with a standard schema. Exportable to other applications, browser history formats, or P2P networks.

Phases

Phase 1 (MVP, weeks 1–8): Force-directed graph canvas, WASD navigation, Servo integration, detail windows with connection tabs, JSON save/load, live search. Single-click select + double-click open interaction model.

Phase 1.5 (validation, weeks 9–10): Use Verso for real work. Test interaction model. Evaluate Petgraph. Prioritize Phase 2 based on actual usage patterns.

Phase 2 (sense-making, weeks 11–18): Session management with history view. Optional sidebar. DOM extraction UI. Clustering and grouping. 3D/2D rendering toggle. Level-of-detail rendering. Export to JSON/PNG/HTML.

Phase 3 (ecosystem, weeks 19–24): Browser extension architecture (Chrome/Firefox). Node/graph sharing via URLs. P2P sync (optional, async-first). Tokenization research via `VERSE.md`; implementation deferred.

See `archive_docs/` for detailed phase breakdown, feature specifications, and timeline estimates.

Contributing and license

- See `.github/CONTRIBUTING.md` and `.github/CODE_OF_CONDUCT.md`.
- Dual-licensed: MIT or Apache-2.0.

See `GRAPH_INTERFACE.md` and `VERSE.md` for implementation and research details.
