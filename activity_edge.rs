// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ActivityEdge (struct)
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
use crate::value_specification::ValueSpecification;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ActivityEdge {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    source: String,
    target: String,
    redefined_edge: Vec<String>,
    in_partition: Vec<String>,
    guard: ValueSpecification,
    weight: ValueSpecification,
    interrupts: Option<String>,
    in_structured_node: Option<String>,
    activity: Option<String>,
}

#[wasm_bindgen]
impl ActivityEdge {
    pub fn new(is_leaf: bool, source: String, target: String, guard: ValueSpecification, weight: ValueSpecification) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            source: source,
            target: target,
            redefined_edge: Vec::new(),
            in_partition: Vec::new(),
            guard: guard,
            weight: weight,
            interrupts: None,
            in_structured_node: None,
            activity: None,
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

    /// Returns is_leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Sets is_leaf
    pub fn set_is_leaf(&mut self, value: bool) {
        self.is_leaf = value;
    }

    /// Returns a clone of source
    pub fn source(&self) -> String {
        self.source.clone()
    }

    /// Sets source
    pub fn set_source(&mut self, value: String) {
        self.source = value;
    }

    /// Takes ownership of source, replacing it with an empty string
    pub fn take_source(&mut self) -> String {
        std::mem::take(&mut self.source)
    }

    /// Returns a clone of target
    pub fn target(&self) -> String {
        self.target.clone()
    }

    /// Sets target
    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }

    /// Takes ownership of target, replacing it with an empty string
    pub fn take_target(&mut self) -> String {
        std::mem::take(&mut self.target)
    }

    /// Returns a clone of redefined_edge
    pub fn redefined_edge(&self) -> Vec<String> {
        self.redefined_edge.clone()
    }

    /// Adds an existing ActivityEdge to redefined_edge by ID
    pub fn add_redefined_edge_by_id(&mut self, id: String) {
        self.redefined_edge.push(id);
    }

    /// Clears all items from redefined_edge
    pub fn clear_redefined_edge(&mut self) {
        self.redefined_edge.clear();
    }

    /// Returns a clone of in_partition
    pub fn in_partition(&self) -> Vec<String> {
        self.in_partition.clone()
    }

    /// Adds an existing ActivityPartition to in_partition by ID
    pub fn add_in_partition_by_id(&mut self, id: String) {
        self.in_partition.push(id);
    }

    /// Clears all items from in_partition
    pub fn clear_in_partition(&mut self) {
        self.in_partition.clear();
    }

    /// Returns a clone of interrupts if present
    pub fn interrupts(&self) -> Option<String> {
        self.interrupts.clone()
    }

    /// Sets interrupts
    pub fn set_interrupts(&mut self, value: String) {
        self.interrupts = Some(value);
    }

    /// Takes interrupts, leaving None in its place
    pub fn take_interrupts(&mut self) -> Option<String> {
        self.interrupts.take()
    }

    /// Returns a clone of in_structured_node if present
    pub fn in_structured_node(&self) -> Option<String> {
        self.in_structured_node.clone()
    }

    /// Sets in_structured_node
    pub fn set_in_structured_node(&mut self, value: String) {
        self.in_structured_node = Some(value);
    }

    /// Takes in_structured_node, leaving None in its place
    pub fn take_in_structured_node(&mut self) -> Option<String> {
        self.in_structured_node.take()
    }

    /// Returns a clone of activity if present
    pub fn activity(&self) -> Option<String> {
        self.activity.clone()
    }

    /// Sets activity
    pub fn set_activity(&mut self, value: String) {
        self.activity = Some(value);
    }

    /// Takes activity, leaving None in its place
    pub fn take_activity(&mut self) -> Option<String> {
        self.activity.take()
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
        "ActivityEdge".to_string()
    }

}

impl Default for ActivityEdge {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            source: Default::default(),
            target: Default::default(),
            redefined_edge: Vec::new(),
            in_partition: Vec::new(),
            guard: Default::default(),
            weight: Default::default(),
            interrupts: None,
            in_structured_node: None,
            activity: None,
        }
    }
}

