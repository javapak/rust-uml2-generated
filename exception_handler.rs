// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ExceptionHandler (struct)
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
pub struct ExceptionHandler {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    handler_body: String,
    exception_input: String,
    exception_type: Vec<String>,
    protected_node: String,
}

#[wasm_bindgen]
impl ExceptionHandler {
    pub fn new(handler_body: String, exception_input: String, exception_type: Vec<String>, protected_node: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            handler_body: handler_body,
            exception_input: exception_input,
            exception_type: exception_type,
            protected_node: protected_node,
        }
    }

    /// Returns a clone of handler_body
    pub fn handler_body(&self) -> String {
        self.handler_body.clone()
    }

    /// Sets handler_body
    pub fn set_handler_body(&mut self, value: String) {
        self.handler_body = value;
    }

    /// Takes ownership of handler_body, replacing it with an empty string
    pub fn take_handler_body(&mut self) -> String {
        std::mem::take(&mut self.handler_body)
    }

    /// Returns a clone of exception_input
    pub fn exception_input(&self) -> String {
        self.exception_input.clone()
    }

    /// Sets exception_input
    pub fn set_exception_input(&mut self, value: String) {
        self.exception_input = value;
    }

    /// Takes ownership of exception_input, replacing it with an empty string
    pub fn take_exception_input(&mut self) -> String {
        std::mem::take(&mut self.exception_input)
    }

    /// Returns a clone of exception_type
    pub fn exception_type(&self) -> Vec<String> {
        self.exception_type.clone()
    }

    /// Adds an existing Classifier to exception_type by ID
    pub fn add_exception_type_by_id(&mut self, id: String) {
        self.exception_type.push(id);
    }

    /// Clears all items from exception_type
    pub fn clear_exception_type(&mut self) {
        self.exception_type.clear();
    }

    /// Returns a clone of protected_node
    pub fn protected_node(&self) -> String {
        self.protected_node.clone()
    }

    /// Sets protected_node
    pub fn set_protected_node(&mut self, value: String) {
        self.protected_node = value;
    }

    /// Takes ownership of protected_node, replacing it with an empty string
    pub fn take_protected_node(&mut self) -> String {
        std::mem::take(&mut self.protected_node)
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
        "ExceptionHandler".to_string()
    }

}

impl Default for ExceptionHandler {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            handler_body: Default::default(),
            exception_input: Default::default(),
            exception_type: Vec::new(),
            protected_node: Default::default(),
        }
    }
}

impl std::fmt::Display for ExceptionHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExceptionHandler(...)")
    }
}

