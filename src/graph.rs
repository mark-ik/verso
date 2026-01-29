use base::id::WebViewId;
use serde::{Deserialize, Serialize};
use webrender_api::units::{DevicePoint, DeviceRect};
use euclid::Point2D;

/// A node in the graph canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique identifier for the node
    pub id: String,
    /// Label/title of the node
    pub label: String,
    /// Position of the node in canvas coordinates
    pub position: Point2D<f32, DevicePoint>,
    /// Size of the node
    pub size: (f32, f32),
    /// Optional WebView ID if a webview is displayed in this node
    pub webview_id: Option<WebViewId>,
    /// Whether the node is currently selected
    pub selected: bool,
}

impl GraphNode {
    /// Create a new graph node
    pub fn new(id: String, label: String, x: f32, y: f32) -> Self {
        Self {
            id,
            label,
            position: Point2D::new(x, y),
            size: (200.0, 100.0), // Default size
            webview_id: None,
            selected: false,
        }
    }

    /// Check if a point is inside this node
    pub fn contains_point(&self, point: &DevicePoint) -> bool {
        let x = point.x;
        let y = point.y;
        x >= self.position.x
            && x <= self.position.x + self.size.0
            && y >= self.position.y
            && y <= self.position.y + self.size.1
    }

    /// Get the bounding rectangle for this node
    pub fn get_rect(&self) -> DeviceRect {
        DeviceRect::from_origin_and_size(
            DevicePoint::new(self.position.x, self.position.y),
            euclid::Size2D::new(self.size.0, self.size.1),
        )
    }
}

/// Manager for the graph canvas
pub struct GraphManager {
    /// List of nodes in the graph
    nodes: Vec<GraphNode>,
    /// Whether the graph view is active
    active: bool,
}

impl GraphManager {
    /// Create a new graph manager
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            active: false,
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut GraphNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get a node by position (for click detection)
    pub fn get_node_at_position(&self, point: &DevicePoint) -> Option<&GraphNode> {
        self.nodes.iter().rev().find(|n| n.contains_point(point))
    }

    /// Get all nodes
    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes
    }

    /// Check if the graph is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set graph active state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Remove a node by ID
    pub fn remove_node(&mut self, id: &str) -> Option<GraphNode> {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            Some(self.nodes.remove(pos))
        } else {
            None
        }
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

impl Default for GraphManager {
    fn default() -> Self {
        Self::new()
    }
}
