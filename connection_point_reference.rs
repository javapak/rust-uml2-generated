// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ConnectionPointReference (struct)
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
pub struct ConnectionPointReference {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    container: Option<String>,
    entry: Vec<String>,
    exit: Vec<String>,
    state: Option<String>,
}

#[wasm_bindgen]
impl ConnectionPointReference {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            container: None,
            entry: Vec::new(),
            exit: Vec::new(),
            state: None,
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

    /// Returns a clone of entry
    pub fn entry(&self) -> Vec<String> {
        self.entry.clone()
    }

    /// Adds an existing Pseudostate to entry by ID
    pub fn add_entry_by_id(&mut self, id: String) {
        self.entry.push(id);
    }

    /// Clears all items from entry
    pub fn clear_entry(&mut self) {
        self.entry.clear();
    }

    /// Returns a clone of exit
    pub fn exit(&self) -> Vec<String> {
        self.exit.clone()
    }

    /// Adds an existing Pseudostate to exit by ID
    pub fn add_exit_by_id(&mut self, id: String) {
        self.exit.push(id);
    }

    /// Clears all items from exit
    pub fn clear_exit(&mut self) {
        self.exit.clear();
    }

    /// Returns a clone of state if present
    pub fn state(&self) -> Option<String> {
        self.state.clone()
    }

    /// Sets state
    pub fn set_state(&mut self, value: String) {
        self.state = Some(value);
    }

    /// Takes state, leaving None in its place
    pub fn take_state(&mut self) -> Option<String> {
        self.state.take()
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
        "ConnectionPointReference".to_string()
    }

}

impl std::fmt::Display for ConnectionPointReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

