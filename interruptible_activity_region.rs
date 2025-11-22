// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           InterruptibleActivityRegion (struct)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-22 12:14:07
// Generator:      EcoreToRustGenerator v0.1.0
//
// Generation Options:
//   - WASM:       enabled
//   - Tsify:      disabled
//   - Serde:      enabled
//   - Builders:   disabled
//   - References: String IDs
//
// WARNING: This file is auto-generated. Manual changes will be overwritten.
// ============================================================================

use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct InterruptibleActivityRegion {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    in_activity: Option<String>,
    node: Vec<String>,
    interrupting_edge: Vec<String>,
}

#[wasm_bindgen]
impl InterruptibleActivityRegion {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            in_activity: None,
            node: Vec::new(),
            interrupting_edge: Vec::new(),
        }
    }

    /// Returns a clone of in_activity if present
    pub fn in_activity(&self) -> Option<String> {
        self.in_activity.clone()
    }

    /// Sets in_activity
    pub fn set_in_activity(&mut self, value: String) {
        self.in_activity = Some(value);
    }

    /// Takes in_activity, leaving None in its place
    pub fn take_in_activity(&mut self) -> Option<String> {
        self.in_activity.take()
    }

    /// Returns a clone of node
    pub fn node(&self) -> Vec<String> {
        self.node.clone()
    }

    /// Adds an existing ActivityNode to node by ID
    pub fn add_node_by_id(&mut self, id: String) {
        self.node.push(id);
    }

    /// Clears all items from node
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    /// Returns a clone of interrupting_edge
    pub fn interrupting_edge(&self) -> Vec<String> {
        self.interrupting_edge.clone()
    }

    /// Adds an existing ActivityEdge to interrupting_edge by ID
    pub fn add_interrupting_edge_by_id(&mut self, id: String) {
        self.interrupting_edge.push(id);
    }

    /// Clears all items from interrupting_edge
    pub fn clear_interrupting_edge(&mut self) {
        self.interrupting_edge.clear();
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|e| e.to_string())
    }

    /// Deserialize from JSON string
    pub fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
    }

    /// Returns whether this type can be created standalone (not nested)
    pub fn can_exist_standalone() -> bool {
        true
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        false
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "InterruptibleActivityRegion".to_string()
    }

}

impl std::fmt::Display for InterruptibleActivityRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterruptibleActivityRegion(...)")
    }
}

