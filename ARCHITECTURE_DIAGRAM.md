# Graph UI Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Verso Window                             │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     Window (window.rs)                      │ │
│  │                                                              │ │
│  │  ┌──────────────────────┐  ┌────────────────────────────┐  │ │
│  │  │   TabManager         │  │    GraphManager            │  │ │
│  │  │  - Tabs              │  │   - Nodes (GraphNode[])    │  │ │
│  │  │  - WebViews          │  │   - Active state           │  │ │
│  │  └──────────────────────┘  └────────────────────────────┘  │ │
│  │                                                              │ │
│  │  ┌──────────────────────┐  ┌────────────────────────────┐  │ │
│  │  │   Panel              │  │    Graph Panel             │  │ │
│  │  │  (Navigation UI)     │  │   (Graph Canvas UI)        │  │ │
│  │  │  panel.html          │  │   graph.html               │  │ │
│  │  └──────────────────────┘  └────────────────────────────┘  │ │
│  │                                                              │ │
│  │  Event Handling:                                             │ │
│  │  - Keyboard shortcuts (Ctrl+G toggles graph)                │ │
│  │  - Mouse events (forwarded to webviews)                     │ │
│  │  - Servo messages (prompts from graph.html)                 │ │
│  └──────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                   Graph Canvas (graph.html)                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     HTML/CSS/JavaScript                     │ │
│  │                                                              │ │
│  │  Canvas Area:                                                │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │ │
│  │  │  Node 1      │  │  Node 2      │  │  Node 3      │      │ │
│  │  │  Label       │  │  Label       │  │  Label       │      │ │
│  │  │  (draggable) │  │  (draggable) │  │  (draggable) │      │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘      │ │
│  │                                                              │ │
│  │  Interactions:                                               │ │
│  │  - Click & drag to move nodes                               │ │
│  │  - Double-click → window.prompt('OPEN_NODE_WEBVIEW:id')    │ │
│  │  - Click '+' button to add new node                         │ │
│  └──────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                     Data Flow Diagram                            │
└─────────────────────────────────────────────────────────────────┘

User presses Ctrl+G
        │
        ▼
Window::handle_keyboard_shortcut()
        │
        ▼
Window::toggle_graph_view()
        │
        ├─► GraphManager::set_active(true)
        │
        └─► Window::create_graph_panel()
                    │
                    └─► Creates Panel with graph.html
                            │
                            └─► Servo constellation loads webview
                                        │
                                        ▼
                                  graph.html renders

User double-clicks node
        │
        ▼
JavaScript: window.prompt('OPEN_NODE_WEBVIEW:node1')
        │
        ▼
Servo sends EmbedderMsg::ShowSimpleDialog::Prompt
        │
        ▼
Window::handle_servo_messages_with_graph_panel()
        │
        ├─► Validates node_id
        │
        └─► Window::handle_graph_node_double_click()
                    │
                    ├─► Creates new WebViewId
                    ├─► Updates GraphNode.webview_id
                    │
                    └─► Sends NewWebView to constellation
                                │
                                ▼
                          Servo creates webview

┌─────────────────────────────────────────────────────────────────┐
│                    Component Relationships                       │
└─────────────────────────────────────────────────────────────────┘

src/graph.rs
    ├─► GraphNode
    │   ├─ id: String
    │   ├─ label: String
    │   ├─ position: DevicePoint
    │   ├─ size: (f32, f32)
    │   └─ webview_id: Option<WebViewId>
    │
    └─► GraphManager
        ├─ nodes: Vec<GraphNode>
        ├─ active: bool
        ├─ add_node()
        ├─ remove_node()
        ├─ get_node_at_position()
        └─ clear()

src/window.rs
    └─► Window
        ├─ graph_manager: GraphManager
        ├─ graph_panel: Option<Panel>
        ├─ toggle_graph_view()
        ├─ create_graph_panel()
        ├─ handle_graph_node_double_click()
        └─ handle_servo_messages_with_graph_panel()

resources/components/graph.html
    └─► JavaScript
        ├─ nodes[] array
        ├─ nodeCounter
        ├─ addNode()
        ├─ renderNode()
        ├─ openWebView() → prompt
        └─ Event handlers (click, drag, etc)

┌─────────────────────────────────────────────────────────────────┐
│                      Message Flow                                │
└─────────────────────────────────────────────────────────────────┘

Window ←──────────────────────────► Servo Constellation
   │                                        │
   │  NewWebView(graph_panel_id)          │
   │ ─────────────────────────────────────>│
   │                                        │
   │  ShowSimpleDialog::Prompt             │
   │ <─────────────────────────────────────│
   │  ("OPEN_NODE_WEBVIEW:node1")         │
   │                                        │
   │  NewWebView(webview_id, url)         │
   │ ─────────────────────────────────────>│
   │                                        │
   ▼                                        ▼
GraphManager                        WebView Pipeline
   │                                        │
   └─► Updates node.webview_id             └─► Renders content

┌─────────────────────────────────────────────────────────────────┐
│                     File Structure                               │
└─────────────────────────────────────────────────────────────────┘

verso/
├── src/
│   ├── graph.rs                  ← NEW: Graph data structures
│   ├── window.rs                 ← MODIFIED: Added graph support
│   ├── lib.rs                    ← MODIFIED: Export graph module
│   └── webview/
│       └── webview.rs            ← (unchanged)
│
├── resources/
│   └── components/
│       ├── graph.html            ← NEW: Graph canvas UI
│       ├── panel.html            ← (unchanged)
│       └── ...
│
├── GRAPH_UI.md                   ← NEW: Technical documentation
├── IMPLEMENTATION_SUMMARY.md     ← NEW: Project summary
└── README.md                     ← MODIFIED: Feature overview
```
