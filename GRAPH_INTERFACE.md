## Graph interface and interaction model

> **Note:** This document describes planned features for the spatial graph browser. The interaction model, physics presets, and rendering system detailed here are design specifications, not current implementation.

Purpose
- Define the interaction model, physics presets, rendering and camera expectations, and short implementation notes for the graph canvas.

Interaction model
- Single left click: select node (Shift+click for multi-select; drag to marquee-select).
- Double left click: open node in a detail window.
- Right click: context menu (create edge, delete, pin, inspect).
- Pan: WASD, arrow keys or middle-mouse drag. 
- Zoom: mouse wheel or +/- keys.

Physics presets
- Liquid (default): strong repulsion, low spring, low damping — organic clustering.
- Solid: moderate repulsion, high spring, high damping — stable manual layouts.
- Gas: large-radius repulsion, very low spring, very low damping — sparse layouts.

Rendering and camera
- Renderer: wgpu-based GPU renderer for nodes, edges, labels, and picking.
- Camera: smooth pan/zoom transforms, bounds clamping, and LOD thresholds that aggregate nodes at zoomed-out levels.

UI patterns and windows
- Detail windows: show a live webview texture for the selected node; connection tabs list adjacent nodes and support in-window navigation.
- Sidebar (Phase 2): session manager, open nodes list, filters/tags, and graph statistics.

Keybindings (core)
- `N`: new node
- `R`: delete selected
- `T`: toggle physics
- `Ctrl+S`: save graph
- `Ctrl+O`: open graph
- `Ctrl+F`: search

Implementation notes
- Start with `egui` for MVP UI; implement a `UIBackend` trait so the UI can be swapped (e.g., Xilem+Vello later).
- Keep the graph core as a separate crate (`verso-graph-core`) containing graph data structures, physics, renderer primitives, camera, and serialization.
- Evaluate `petgraph` in an experimental branch (Phase 1.5) for algorithmic features; retain a custom implementation if tighter control is required.
- WebView orchestration: MVP uses an in-process pool (Servo). Consider extracting a helper process with JSON IPC (Tao+Wry) if stability requires process isolation.

Browser extensions and interoperability

- **Embedding in extensions**: Chrome/Firefox extensions embed the graph canvas via the `UIBackend` trait. A minimal extension crate reuses `verso-ui` and `verso-graph-core`, rendering the canvas in a pop-up or new tab.
- **History import and export**: Verso exports graphs as JSON with node metadata (URL, title, timestamp, tags, edges). A bridge module maps browser history APIs (Chrome History, Firefox Places) to Verso node format for import. Export is portable to other tools.
- **Portable node format**: Nodes store source URL, favicon, selectors, and custom metadata. Extensions can parse Verso JSON, ensuring data portability across applications.
- **WebView backend for extensions**: Extensions use Tao+Wry (lighter than Servo) via the `BrowserEngine` trait. Desktop Verso uses Servo; the trait decouples implementation choice.

Notes
- Store concrete physics parameters in code with UI-exposed tuning controls.
- Use Phase 1.5 as an intentional validation period to refine interaction choices based on actual usage.
- Minimize extension boilerplate; most logic lives in reusable core crates.
