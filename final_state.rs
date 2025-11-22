// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           FinalState (struct)
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
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::region::Region;
use crate::connection_point_reference::ConnectionPointReference;
use crate::pseudostate::Pseudostate;
use crate::behavior::Behavior;
use crate::trigger::Trigger;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct FinalState {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
    is_leaf: bool,
    container: Option<String>,
    submachine: Option<String>,
    connection: Vec<ConnectionPointReference>,
    connection_point: Vec<Pseudostate>,
    redefined_state: Option<String>,
    state_invariant: Option<Constraint>,
    entry: Option<Behavior>,
    exit: Option<Behavior>,
    do_activity: Option<Behavior>,
    deferrable_trigger: Vec<Trigger>,
    region: Vec<Region>,
}

#[wasm_bindgen]
impl FinalState {
    pub fn new(is_leaf: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            is_leaf: is_leaf,
            container: None,
            submachine: None,
            connection: Vec::new(),
            connection_point: Vec::new(),
            redefined_state: None,
            state_invariant: None,
            entry: None,
            exit: None,
            do_activity: None,
            deferrable_trigger: Vec::new(),
            region: Vec::new(),
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

    /// Returns a clone of container if present
    pub fn container(&self) -> Option<String> {
        self.container.clone()
    }

    /// Sets container
    pub fn set_container(&mut self, value: String) {
        self.container = Some(value);
    }

    /// Takes container, leaving None in its place
    pub fn take_container(&mut self) -> Option<String> {
        self.container.take()
    }

    /// Returns a clone of submachine if present
    pub fn submachine(&self) -> Option<String> {
        self.submachine.clone()
    }

    /// Sets submachine
    pub fn set_submachine(&mut self, value: String) {
        self.submachine = Some(value);
    }

    /// Takes submachine, leaving None in its place
    pub fn take_submachine(&mut self) -> Option<String> {
        self.submachine.take()
    }

    /// Returns a clone of redefined_state if present
    pub fn redefined_state(&self) -> Option<String> {
        self.redefined_state.clone()
    }

    /// Sets redefined_state
    pub fn set_redefined_state(&mut self, value: String) {
        self.redefined_state = Some(value);
    }

    /// Takes redefined_state, leaving None in its place
    pub fn take_redefined_state(&mut self) -> Option<String> {
        self.redefined_state.take()
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
        "FinalState".to_string()
    }

}

impl std::fmt::Display for FinalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

