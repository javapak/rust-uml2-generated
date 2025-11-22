// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ActivityPartition (struct)
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
use crate::string_expression::StringExpression;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct ActivityPartition {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    in_activity: Option<String>,
    is_dimension: bool,
    is_external: bool,
    node: Vec<String>,
    subpartition: Vec<Box<ActivityPartition>>,
    super_partition: Option<String>,
    represents: Option<String>,
    edge: Vec<String>,
}

#[wasm_bindgen]
impl ActivityPartition {
    pub fn new(is_dimension: bool, is_external: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            in_activity: None,
            is_dimension: is_dimension,
            is_external: is_external,
            node: Vec::new(),
            subpartition: Vec::new(),
            super_partition: None,
            represents: None,
            edge: Vec::new(),
        }
    }

    /// Returns a clone of name if present
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a clone of visibility if present
    pub fn visibility(&self) -> Option<String> {
        self.visibility.clone()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = Some(value);
    }

    /// Takes visibility, leaving None in its place
    pub fn take_visibility(&mut self) -> Option<String> {
        self.visibility.take()
    }

    /// Returns a clone of client_dependency
    pub fn client_dependency(&self) -> Vec<String> {
        self.client_dependency.clone()
    }

    /// Adds an existing Dependency to client_dependency by ID
    pub fn add_client_dependency_by_id(&mut self, id: String) {
        self.client_dependency.push(id);
    }

    /// Clears all items from client_dependency
    pub fn clear_client_dependency(&mut self) {
        self.client_dependency.clear();
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

    /// Returns is_dimension
    pub fn is_dimension(&self) -> bool {
        self.is_dimension
    }

    /// Sets is_dimension
    pub fn set_is_dimension(&mut self, value: bool) {
        self.is_dimension = value;
    }

    /// Returns is_external
    pub fn is_external(&self) -> bool {
        self.is_external
    }

    /// Sets is_external
    pub fn set_is_external(&mut self, value: bool) {
        self.is_external = value;
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

    /// Returns a clone of super_partition if present
    pub fn super_partition(&self) -> Option<String> {
        self.super_partition.clone()
    }

    /// Sets super_partition
    pub fn set_super_partition(&mut self, value: String) {
        self.super_partition = Some(value);
    }

    /// Takes super_partition, leaving None in its place
    pub fn take_super_partition(&mut self) -> Option<String> {
        self.super_partition.take()
    }

    /// Returns a clone of represents if present
    pub fn represents(&self) -> Option<String> {
        self.represents.clone()
    }

    /// Sets represents
    pub fn set_represents(&mut self, value: String) {
        self.represents = Some(value);
    }

    /// Takes represents, leaving None in its place
    pub fn take_represents(&mut self) -> Option<String> {
        self.represents.take()
    }

    /// Returns a clone of edge
    pub fn edge(&self) -> Vec<String> {
        self.edge.clone()
    }

    /// Adds an existing ActivityEdge to edge by ID
    pub fn add_edge_by_id(&mut self, id: String) {
        self.edge.push(id);
    }

    /// Clears all items from edge
    pub fn clear_edge(&mut self) {
        self.edge.clear();
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
        "ActivityPartition".to_string()
    }

}

impl std::fmt::Display for ActivityPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

