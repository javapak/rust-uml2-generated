// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Clause (struct)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Clause {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    test: Vec<String>,
    body: Vec<String>,
    predecessor_clause: Vec<String>,
    successor_clause: Vec<String>,
    decider: String,
    body_output: Vec<String>,
}

#[wasm_bindgen]
impl Clause {
    pub fn new(decider: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            test: Vec::new(),
            body: Vec::new(),
            predecessor_clause: Vec::new(),
            successor_clause: Vec::new(),
            decider: decider,
            body_output: Vec::new(),
        }
    }

    /// Returns a clone of test
    pub fn test(&self) -> Vec<String> {
        self.test.clone()
    }

    /// Adds an existing ExecutableNode to test by ID
    pub fn add_test_by_id(&mut self, id: String) {
        self.test.push(id);
    }

    /// Clears all items from test
    pub fn clear_test(&mut self) {
        self.test.clear();
    }

    /// Returns a clone of body
    pub fn body(&self) -> Vec<String> {
        self.body.clone()
    }

    /// Adds an existing ExecutableNode to body by ID
    pub fn add_body_by_id(&mut self, id: String) {
        self.body.push(id);
    }

    /// Clears all items from body
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    /// Returns a clone of predecessor_clause
    pub fn predecessor_clause(&self) -> Vec<String> {
        self.predecessor_clause.clone()
    }

    /// Adds an existing Clause to predecessor_clause by ID
    pub fn add_predecessor_clause_by_id(&mut self, id: String) {
        self.predecessor_clause.push(id);
    }

    /// Clears all items from predecessor_clause
    pub fn clear_predecessor_clause(&mut self) {
        self.predecessor_clause.clear();
    }

    /// Returns a clone of successor_clause
    pub fn successor_clause(&self) -> Vec<String> {
        self.successor_clause.clone()
    }

    /// Adds an existing Clause to successor_clause by ID
    pub fn add_successor_clause_by_id(&mut self, id: String) {
        self.successor_clause.push(id);
    }

    /// Clears all items from successor_clause
    pub fn clear_successor_clause(&mut self) {
        self.successor_clause.clear();
    }

    /// Returns a clone of decider
    pub fn decider(&self) -> String {
        self.decider.clone()
    }

    /// Sets decider
    pub fn set_decider(&mut self, value: String) {
        self.decider = value;
    }

    /// Takes ownership of decider, replacing it with an empty string
    pub fn take_decider(&mut self) -> String {
        std::mem::take(&mut self.decider)
    }

    /// Returns a clone of body_output
    pub fn body_output(&self) -> Vec<String> {
        self.body_output.clone()
    }

    /// Adds an existing OutputPin to body_output by ID
    pub fn add_body_output_by_id(&mut self, id: String) {
        self.body_output.push(id);
    }

    /// Clears all items from body_output
    pub fn clear_body_output(&mut self) {
        self.body_output.clear();
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
        false
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        true
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "Clause".to_string()
    }

}

impl Default for Clause {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            test: Vec::new(),
            body: Vec::new(),
            predecessor_clause: Vec::new(),
            successor_clause: Vec::new(),
            decider: Default::default(),
            body_output: Vec::new(),
        }
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clause(...)")
    }
}

