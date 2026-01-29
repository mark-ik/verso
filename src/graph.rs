use base::id::WebViewId;
use serde::{Deserialize, Serialize};
use webrender_api::units::{DevicePoint, DeviceRect};

/// A node in the graph canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique identifier for the node
    pub id: String,
    /// Label/title of the node
    pub label: String,
    /// Position of the node in canvas coordinates (DevicePoint is already Point2D<f32, DevicePixel>)
    pub position: DevicePoint,
    /// Size of the node
    pub size: (f32, f32),
    /// Optional WebView ID if a webview is displayed in this node
    pub webview_id: Option<WebViewId>,
}

impl GraphNode {
    /// Create a new graph node
    pub fn new(id: String, label: String, x: f32, y: f32) -> Self {
        Self {
            id,
            label,
            position: DevicePoint::new(x, y),
            size: (200.0, 100.0), // Default size
            webview_id: None,
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
            self.position,
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
    /// Returns an error if a node with the same ID already exists
    pub fn add_node(&mut self, node: GraphNode) -> Result<(), String> {
        if self.nodes.iter().any(|n| n.id == node.id) {
            return Err(format!("Node with id '{}' already exists", node.id));
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut GraphNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get a node by position (for click detection)
    pub fn get_node_at_position(&self, point: &DevicePoint) -> Option<&GraphNode> {
        // TODO: For large graphs, consider spatial indexing (quadtree/grid) for better performance
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

    /// Remove a node by ID and return its webview ID if it had one
    /// Caller is responsible for sending CloseWebView message to constellation
    pub fn remove_node(&mut self, id: &str) -> Option<(GraphNode, Option<WebViewId>)> {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            let node = self.nodes.remove(pos);
            let webview_id = node.webview_id;
            Some((node, webview_id))
        } else {
            None
        }
    }

    /// Clear all nodes and return any webview IDs that need cleanup
    /// Caller is responsible for sending CloseWebView messages to constellation
    pub fn clear(&mut self) -> Vec<WebViewId> {
        let webview_ids: Vec<WebViewId> = self.nodes
            .iter()
            .filter_map(|n| n.webview_id)
            .collect();
        self.nodes.clear();
        webview_ids
    }
}

impl Default for GraphManager {
    fn default() -> Self {
        Self::new()
    }
}
