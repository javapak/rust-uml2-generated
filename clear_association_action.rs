// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ClearAssociationAction (struct)
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
use crate::exception_handler::ExceptionHandler;
use crate::constraint::Constraint;
use crate::input_pin::InputPin;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ClearAssociationAction {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    in_structured_node: Option<String>,
    activity: Option<String>,
    outgoing: Vec<String>,
    incoming: Vec<String>,
    in_partition: Vec<String>,
    in_interruptible_region: Vec<String>,
    redefined_node: Vec<String>,
    handler: Vec<ExceptionHandler>,
    local_precondition: Vec<Constraint>,
    local_postcondition: Vec<Constraint>,
    object: InputPin,
    association: String,
}

#[wasm_bindgen]
impl ClearAssociationAction {
    pub fn new(is_leaf: bool, object: InputPin, association: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            object: object,
            association: association,
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

    /// Returns a clone of outgoing
    pub fn outgoing(&self) -> Vec<String> {
        self.outgoing.clone()
    }

    /// Adds an existing ActivityEdge to outgoing by ID
    pub fn add_outgoing_by_id(&mut self, id: String) {
        self.outgoing.push(id);
    }

    /// Clears all items from outgoing
    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }

    /// Returns a clone of incoming
    pub fn incoming(&self) -> Vec<String> {
        self.incoming.clone()
    }

    /// Adds an existing ActivityEdge to incoming by ID
    pub fn add_incoming_by_id(&mut self, id: String) {
        self.incoming.push(id);
    }

    /// Clears all items from incoming
    pub fn clear_incoming(&mut self) {
        self.incoming.clear();
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

    /// Returns a clone of in_interruptible_region
    pub fn in_interruptible_region(&self) -> Vec<String> {
        self.in_interruptible_region.clone()
    }

    /// Adds an existing InterruptibleActivityRegion to in_interruptible_region by ID
    pub fn add_in_interruptible_region_by_id(&mut self, id: String) {
        self.in_interruptible_region.push(id);
    }

    /// Clears all items from in_interruptible_region
    pub fn clear_in_interruptible_region(&mut self) {
        self.in_interruptible_region.clear();
    }

    /// Returns a clone of redefined_node
    pub fn redefined_node(&self) -> Vec<String> {
        self.redefined_node.clone()
    }

    /// Adds an existing ActivityNode to redefined_node by ID
    pub fn add_redefined_node_by_id(&mut self, id: String) {
        self.redefined_node.push(id);
    }

    /// Clears all items from redefined_node
    pub fn clear_redefined_node(&mut self) {
        self.redefined_node.clear();
    }

    /// Returns a clone of association
    pub fn association(&self) -> String {
        self.association.clone()
    }

    /// Sets association
    pub fn set_association(&mut self, value: String) {
        self.association = value;
    }

    /// Takes ownership of association, replacing it with an empty string
    pub fn take_association(&mut self) -> String {
        std::mem::take(&mut self.association)
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
        "ClearAssociationAction".to_string()
    }

}

impl Default for ClearAssociationAction {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            object: Default::default(),
            association: Default::default(),
        }
    }
}

impl std::fmt::Display for ClearAssociationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

