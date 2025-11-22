// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EAnnotation (struct)
// Source Package: ecore
// Package URI:    http://www.eclipse.org/emf/2002/Ecore
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

use crate::eobject::EObject;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EAnnotation {
    e_annotations: Vec<Box<EAnnotation>>,
    source: Option<String>,
    contents: Vec<EObject>,
    references: Vec<String>,
}

#[wasm_bindgen]
impl EAnnotation {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            source: None,
            contents: Vec::new(),
            references: Vec::new(),
        }
    }

    /// Returns a clone of source if present
    pub fn source(&self) -> Option<String> {
        self.source.clone()
    }

    /// Sets source
    pub fn set_source(&mut self, value: String) {
        self.source = Some(value);
    }

    /// Takes source, leaving None in its place
    pub fn take_source(&mut self) -> Option<String> {
        self.source.take()
    }

    /// Returns a clone of references
    pub fn references(&self) -> Vec<String> {
        self.references.clone()
    }

    /// Adds an existing EObject to references by ID
    pub fn add_reference_by_id(&mut self, id: String) {
        self.references.push(id);
    }

    /// Clears all items from references
    pub fn clear_references(&mut self) {
        self.references.clear();
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
        "EAnnotation".to_string()
    }

}

impl std::fmt::Display for EAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EAnnotation(...)")
    }
}

