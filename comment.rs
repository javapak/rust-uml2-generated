// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Comment (struct)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-22 12:14:06
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
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Comment {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Box<Comment>>,
    body: Option<String>,
    annotated_element: Vec<String>,
}

#[wasm_bindgen]
impl Comment {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            body: None,
            annotated_element: Vec::new(),
        }
    }

    /// Returns a clone of body if present
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }

    /// Sets body
    pub fn set_body(&mut self, value: String) {
        self.body = Some(value);
    }

    /// Takes body, leaving None in its place
    pub fn take_body(&mut self) -> Option<String> {
        self.body.take()
    }

    /// Returns a clone of annotated_element
    pub fn annotated_element(&self) -> Vec<String> {
        self.annotated_element.clone()
    }

    /// Adds an existing Element to annotated_element by ID
    pub fn add_annotated_element_by_id(&mut self, id: String) {
        self.annotated_element.push(id);
    }

    /// Clears all items from annotated_element
    pub fn clear_annotated_element(&mut self) {
        self.annotated_element.clear();
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
        "Comment".to_string()
    }

}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comment(...)")
    }
}

