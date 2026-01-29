# Archived: Architecture Modular Analysis

This architecture analysis has been archived. See the archived copy below for the original detailed modular architecture notes.

Archived copy: [archive_docs/ARCHITECTURE_MODULAR_ANALYSIS.md](archive_docs/ARCHITECTURE_MODULAR_ANALYSIS.md)

For the current modular decisions and short architecture summary see [README.md](README.md) and [GRAPH_INTERFACE.md](GRAPH_INTERFACE.md).
verso-servo-browser (binary crate - consumes verso-graph-core)
├── browser/        (Servo integration)
├── ui/             (egui overlay for chrome/menus)
└── main.rs

verso-chrome-extension (binary crate - consumes verso-graph-core)
├── browser/        (Chrome tab API integration)
└── ui/             (native Chrome extension UI)

verso-visualization (library crate - consumes verso-graph-core)
└── (Generic spatial graph visualization for any data)
```

**Characteristics**:
- Graph core is **platform-agnostic, pure library**
- Browser engine abstracted via traits
- UI framework swappable (egui → GPUI → Xilem)
- Reusable in extensions, other apps, data visualization tools
- Multiple targets (desktop, extension, library consumers)

---

## Key Architectural Decisions

### 1. **Graph Core ≠ Browser Engine** 🔥 **MAJOR CHANGE**

**Current plan**: Graph engine lives in `src/graph/` alongside Servo integration.

**Proposed**: Extract graph core into separate `verso-graph-core` crate with:
- **Zero dependencies** on:
  - Servo, WebRender, compositor logic
  - Platform-specific window code
  - UI frameworks
- **Pure dependencies**:
  - `wgpu` (GPU rendering)
  - `nalgebra` or `glam` (math)
  - `serde` (serialization)
  - Standard library

**Advantage**: Graph library can be published on crates.io, used independently.

**Trade-off**: Requires architectural discipline; can't import Servo types into core.

---

### 2. **Browser Engine as Trait** 🔥 **MAJOR CHANGE**

**Proposed trait**:
```rust
// verso-graph-core/src/browser.rs
pub trait BrowserEngine {
    type WebView;
    
    fn create_webview(&mut self, node_id: NodeId, url: &str) -> Result<Self::WebView>;
    fn load_url(&mut self, webview: &mut Self::WebView, url: &str) -> Result<()>;
    fn render_to_texture(&mut self, webview: &Self::WebView) -> TextureHandle;
    fn cleanup_webview(&mut self, webview: Self::WebView) -> Result<()>;
}
```

**Implementations**:

| Engine | Crate | Status |
|--------|-------|--------|
| Servo | `verso-servo-browser` | MVP (current plan) |
| Chrome Tab API | `verso-chrome-extension` | Phase 3+ |
| Firefox Tab API | `verso-firefox-extension` | Phase 3+ |
| Headless (testing) | `verso-graph-core` (test) | MVP (for unit tests) |

**Advantage**: Same graph code works with different browser engines.

**Trade-off**: Requires abstracting Servo-specific APIs (Constellation messages, embedder_traits, etc.)

---

### 3. **Rendering as a GPU Pipeline** 🔥 **CHANGE**

**Current plan**: WebRender compositor → Verso window.

**Proposed**: 
- `verso-graph-core` uses **wgpu** for graph rendering (independent of Servo)
- WebRender still renders webpages to textures (Servo responsibility)
- Graph core composes its own quad-based rendering (nodes + edges + HUD)
- UI framework (egui) renders chrome/menus on top

**Benefit**: Graph core doesn't depend on Servo's WebRender; can work with Chrome's rendering pipeline too.

**Implementation**:
```rust
// verso-graph-core/src/renderer.rs
pub struct GraphRenderer {
    wgpu_device: wgpu::Device,
    wgpu_queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    // ...
}

impl GraphRenderer {
    pub fn render_frame(
        &mut self,
        graph: &Graph,
        camera: &Camera,
        node_textures: &HashMap<NodeId, TextureHandle>,
    ) -> wgpu::Texture {
        // Render nodes (as quads with node_textures)
        // Render edges (as lines)
        // Render HUD (text, FPS counter, etc.)
    }
}
```

---

### 4. **UI Framework Abstraction** 🔥 **CHANGE**

**Current plan**: Verso's chrome/menus are closely tied to one UI system.

**Proposed**: Define traits for UI operations:

```rust
// verso-graph-core/src/ui_traits.rs
pub trait UIBackend {
    fn render_search_bar(&mut self) -> Option<SearchQuery>;
    fn render_settings_dialog(&mut self) -> Option<SettingsUpdate>;
    fn render_menu(&mut self) -> Option<MenuAction>;
    fn render_status_bar(&mut self, stats: &Stats) -> Result<()>;
}
```

**Implementations**:
- `verso-ui-egui`: egui-based (MVP, initial PoC)
- `verso-ui-gpui`: GPUI-based (future)
- `verso-ui-xilem`: Xilem-based (future)
- `verso-ui-web`: Web-based (browser extension)

**Advantage**: Swap UI frameworks without changing graph logic.

---

## Feasibility & Trade-offs

### Advantages of Modular Architecture ✅

1. **Publishable graph library**
   - `verso-graph-core` on crates.io
   - General-purpose spatial graph visualization
   - Not browser-specific; anyone can use it

2. **Browser extension pathway**
   - Chrome/Firefox extensions can consume graph core
   - Reuse physics + rendering + graph structure
   - Use browser's native tab API instead of Servo

3. **Testability**
   - Graph core has zero dependencies (except math/serde)
   - Can unit test physics, algorithms, serialization in isolation
   - No need to spin up Servo to test graph logic

4. **Code reuse**
   - Same graph library in desktop app + extension + other projects
   - Reduces duplication

5. **Swappable components**
   - Could use different browser engines (headless Chrome, Firefox, etc.)
   - Could use different UI frameworks (egui → GPUI)
   - Not locked into one choice

### Disadvantages / Challenges ⚠️

1. **Architecture overhead**
   - Requires careful trait design upfront
   - Abstraction costs (trait dispatch, error handling)
   - More complex to reason about than monolithic design

2. **Delayed MVP**
   - Current plan targets working Verso app in ~8 weeks
   - Modular design adds 2-4 weeks of architecture work
   - Traits need to be designed correctly; wrong design = painful refactor

3. **Servo integration complexity**
   - Servo's APIs are deeply tied to its compositor/constellation
   - Abstracting `CompositorMsg`, `WebViewDelegate`, etc. behind traits is non-trivial
   - May require creating wrapper types

4. **Testing trade-offs**
   - Headless testing easier with modular design
   - But browser-specific testing (Servo integration) still requires full stack

5. **Maintenance burden**
   - More crates to maintain
   - Trait stability becomes critical (breaking changes affect all implementers)

---

## Integration with Current Migration Plan

### Option A: Full Refactor to Modular (Ambitious)

**Phases**:
1. **Phase 1 (Weeks 1–4)**: Extract graph-core
   - Create `verso-graph-core` crate (pure library)
   - Implement graph engine, physics, wgpu rendering
   - Define `BrowserEngine` and `UIBackend` traits
   - Create headless test harness

2. **Phase 1 (Weeks 5–8)**: Implement Servo as trait
   - Create `verso-servo-browser` crate
   - Implement `BrowserEngine` for Servo
   - Create `verso-ui-egui` crate
   - Integrate graph-core + Servo + egui into working app

3. **Phase 2 (Weeks 9–16)**: Features
   - Add features to graph-core (clustering, filtering, persistence)
   - Keep feature logic in core; UI implementations per framework

4. **Phase 3 (Weeks 17–24)**: Extensions
   - Implement `verso-chrome-extension` using graph-core
   - Implement `verso-firefox-extension` using graph-core
   - Tokenization/P2P research (can be framework-agnostic)

**Timeline**: Same 8-week MVP window, but modular structure in place.

**Outcome**: 
- `verso-graph-core` is publishable library
- `verso-servo-browser` is desktop app
- Extensions are possible in Phase 3

### Option B: Keep Current Plan, Refactor Later (Conservative)

**Phases**:
1. **Phase 1–2** (Weeks 1–16): Current migration plan (monolithic)
   - Build graph, rendering, Servo integration tightly coupled
   - Ship working app

2. **Phase 2–3 transition**: Modularization refactor
   - Extract graph-core from monolithic codebase
   - Create traits for browser/UI
   - Separate Servo integration into wrapper crate
   - ~4 weeks of careful refactoring

3. **Phase 3**: Extension development
   - Now can build extensions using extracted graph-core

**Outcome**:
- Working app sooner (no upfront architecture design)
- More refactoring work later (higher risk of breaking things)
- Library extraction delayed to ~week 18–22

---

## Recommendation

### For MVP: **Hybrid Approach (Best of Both)**

**Phase 1 (Weeks 1–8)**: Build modular core, but ship monolithic app

1. **Weeks 1–2**: Design + prototype traits
   - `BrowserEngine`, `UIBackend` traits (on paper)
   - Validate they work for Servo + egui

2. **Weeks 2–5**: Implement graph-core
   - Pure `verso-graph-core` crate with physics, graph, wgpu rendering
   - Zero Servo/UI dependencies
   - Headless test binary (validates core logic)

3. **Weeks 5–8**: Integrate into monolithic Verso app
   - Create `verso` binary that uses graph-core
   - Integrate Servo via trait (rough implementation OK for MVP)
   - Integrate egui via trait (rough implementation OK for MVP)
   - Ship working app

**Phase 2**: Keep monolithic; stabilize features

**Phase 2–3 transition** (~week 16–18): Clean-up refactor
- Formalize `BrowserEngine` and `UIBackend` traits
- Move Servo code into separate `verso-servo-browser` crate
- Extract egui code into separate `verso-ui-egui` crate
- Library becomes publish-ready

**Phase 3**: Extensions
- Build `verso-chrome-extension` on graph-core
- Build `verso-firefox-extension` on graph-core

**Benefits**:
- MVP still ships in 8 weeks (not delayed)
- Architecture is sound from the start (not a surprise refactor)
- Graph-core is ready for publication by week 16–18
- Extension pathway is clear from the beginning
- Less risk than full upfront modularization

---

## Key Files to Create/Modify

### If Going Hybrid Approach:

**New crates**:
```
verso/
├── verso-graph-core/        (NEW - pure library)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── graph/           (nodes, edges, graph container)
│   │   ├── physics/         (force simulation)
│   │   ├── renderer/        (wgpu rendering)
│   │   ├── traits/          (BrowserEngine, UIBackend)
│   │   ├── camera.rs
│   │   └── types.rs         (common types)
│   └── examples/
│       └── headless.rs      (test harness without browser/UI)
│
├── verso/                   (existing, now uses verso-graph-core)
│   ├── Cargo.toml           (add verso-graph-core dependency)
│   ├── src/
│   │   ├── main.rs
│   │   ├── browser/         (Servo BrowserEngine impl)
│   │   ├── ui/              (egui UIBackend impl)
│   │   └── app.rs           (wires graph-core + browser + UI)
│   └── ...
│
├── verso-ui-egui/           (NEW - UI framework impl)
│   └── ...
│
└── verso-servo-browser/     (FUTURE - separate Servo app)
    └── ...
```

**Key design documents**:
- Update `GRAPH_BROWSER_MIGRATION.md` with new architecture section
- Create `ARCHITECTURE.md` explaining trait design + module responsibilities
- Document `BrowserEngine` and `UIBackend` traits

---

## Summary

The sketch notes propose a **library-first, reusable approach** vs. the current **monolithic app approach**.

| Aspect | Current Plan | Sketch Notes |
|--------|---|---|
| **Core artifact** | Verso browser app | verso-graph-core library |
| **Reusability** | Verso-specific only | General spatial graph lib + extensions |
| **Browser engines** | Servo only (hardcoded) | Pluggable (Servo, Chrome API, Firefox API, headless) |
| **UI frameworks** | Single framework | Swappable (egui, GPUI, Xilem) |
| **Extension path** | Deferred; would require rewrite | Enabled from start; reuse graph-core |
| **MVP timeline** | 8 weeks | 8 weeks (with hybrid approach) |
| **Testability** | Full stack required | Graph core testable in isolation |
| **Code reuse** | None (monolithic) | High (library + multiple consumers) |

**Recommendation**: Adopt the **hybrid approach**—design modular traits upfront, implement graph-core as pure library (weeks 1–5), integrate into monolithic Verso app (weeks 5–8), then clean up separation in Phase 2–3 transition.

**This adds minimal MVP delay** (maybe 1 week for architecture upfront) but **unlocks extension pathway and general-purpose library** by Phase 3 start.

