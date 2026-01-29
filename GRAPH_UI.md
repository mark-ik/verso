# Graph UI Implementation

This document describes the graph canvas UI feature added to Verso browser.

## Overview

The graph UI provides a node-based canvas interface where:
- Users can view and manipulate nodes on a canvas
- Double-clicking a node opens a Servo webview displaying web content
- Nodes can be dragged and repositioned
- New nodes can be added dynamically

## Components

### 1. Graph Module (`src/graph.rs`)

#### GraphNode
```rust
pub struct GraphNode {
    pub id: String,           // Unique identifier
    pub label: String,        // Display name
    pub position: Point2D,    // Canvas position
    pub size: (f32, f32),     // Node dimensions
    pub webview_id: Option<WebViewId>, // Associated webview
    pub selected: bool,       // Selection state
}
```

#### GraphManager
Manages the collection of graph nodes and provides:
- `add_node()` - Add a new node
- `get_node_mut()` - Get mutable reference to a node
- `get_node_at_position()` - Hit testing for clicks
- `is_active()` / `set_active()` - Toggle graph view

### 2. Window Integration (`src/window.rs`)

The `Window` struct now includes:
- `graph_manager: GraphManager` - Manages graph state
- `graph_panel: Option<Panel>` - Webview for graph UI

New methods:
- `toggle_graph_view()` - Toggles graph view on/off
- `create_graph_panel()` - Creates the graph canvas webview
- `handle_graph_node_double_click()` - Creates webview when node is double-clicked
- `handle_servo_messages_with_graph_panel()` - Handles graph panel events

### 3. Graph HTML UI (`resources/components/graph.html`)

Interactive HTML/CSS/JavaScript interface with:
- Canvas for rendering nodes
- Node drag-and-drop support
- Double-click detection for opening webviews
- Visual feedback (selection, hover states)
- Instructions overlay

## User Interface

### Keyboard Shortcuts
- **Ctrl+G** (Cmd+G on macOS): Toggle graph view on/off

### Mouse Interactions
- **Click and drag**: Move nodes around the canvas
- **Double-click node**: Open a Servo webview for that node
- **Click + button**: Add a new node to the canvas
- **Click background**: Deselect current node

## Technical Details

### Graph Panel Creation Flow

1. User presses Ctrl+G
2. `toggle_graph_view()` is called
3. If graph panel doesn't exist, `create_graph_panel()` creates it:
   - Generates new WebViewId
   - Creates Panel with graph.html URL
   - Sends NewWebView message to Servo constellation
4. Graph HTML loads and displays canvas

### Node Double-Click Flow

1. User double-clicks node in graph.html
2. JavaScript calls `window.prompt('OPEN_NODE_WEBVIEW:node_id')`
3. Servo sends ShowSimpleDialog::Prompt to embedder
4. `handle_servo_messages_with_graph_panel()` receives message
5. Extracts node_id from prompt message
6. Calls `handle_graph_node_double_click(node_id)`
7. Creates new webview and associates it with the node
8. Sends NewWebView message to constellation with initial URL

### Webview Integration

When a node's webview is created:
- A new `WebViewId` is generated
- Viewport details are calculated based on node size
- Initial URL is set (currently https://servo.org)
- WebViewId is stored in the node for future reference
- Webview is managed by Servo's constellation

## Architecture Decisions

### Why HTML Canvas?
Using an HTML-based UI (graph.html) allows:
- Rapid iteration and styling with CSS
- DOM event handling for interactions
- Consistent with Verso's existing panel approach
- Leverages Servo's rendering capabilities

### Webview-per-Node Design
Each node can have an associated webview:
- Enables displaying web content within nodes
- Maintains Verso's existing webview management
- Allows nodes to load different URLs
- Future: Could support embedding multiple webviews in a split view

## Current State

### Implemented
✅ Graph data structures (GraphNode, GraphManager)
✅ Window integration and state management
✅ Graph panel as HTML webview
✅ Node rendering and interaction
✅ Double-click event handling via prompts
✅ Webview creation on node double-click
✅ Keyboard shortcut (Ctrl+G/Cmd+G)

### Future Enhancements
- Visual display of webview content within nodes
- Connection lines between nodes
- Node persistence (save/load graph state)
- Multiple graph layouts and views
- Zoom and pan controls
- Node properties and metadata
- Drag-to-connect functionality
- Context menus for nodes
- Node templates and presets

## Testing

To test the graph UI:

1. Build and run Verso:
   ```bash
   cargo run
   ```

2. Press **Ctrl+G** (Cmd+G on macOS) to toggle the graph view

3. Try the following interactions:
   - Click and drag nodes to move them
   - Double-click a node to create a webview
   - Click the + button to add new nodes
   - Press Ctrl+G again to hide the graph view

## Known Limitations

1. Webviews are created but not visually embedded in nodes yet
2. Graph state is not persisted across sessions
3. No visual connections between nodes
4. Limited to single graph instance per window

## Integration with Servo

The implementation keeps Servo's current state in mind:
- Uses existing Panel/WebView architecture
- Leverages constellation messaging system
- Compatible with current Servo rendering pipeline
- Minimal changes to core Servo integration code

## Code Structure

```
src/
├── graph.rs                 # Graph data structures and manager
├── window.rs                # Window integration and event handling
└── webview/
    └── webview.rs           # Webview message handling (unchanged)

resources/components/
└── graph.html              # Graph canvas UI
```
