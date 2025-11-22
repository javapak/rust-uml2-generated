// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           QualifierValue (struct)
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
pub struct QualifierValue {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    qualifier: String,
    value: String,
}

#[wasm_bindgen]
impl QualifierValue {
    pub fn new(qualifier: String, value: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            qualifier: qualifier,
            value: value,
        }
    }

    /// Returns a clone of qualifier
    pub fn qualifier(&self) -> String {
        self.qualifier.clone()
    }

    /// Sets qualifier
    pub fn set_qualifier(&mut self, value: String) {
        self.qualifier = value;
    }

    /// Takes ownership of qualifier, replacing it with an empty string
    pub fn take_qualifier(&mut self) -> String {
        std::mem::take(&mut self.qualifier)
    }

    /// Returns a clone of value
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Sets value
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Takes ownership of value, replacing it with an empty string
    pub fn take_value(&mut self) -> String {
        std::mem::take(&mut self.value)
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
        "QualifierValue".to_string()
    }

}

impl Default for QualifierValue {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            qualifier: Default::default(),
            value: Default::default(),
        }
    }
}

impl std::fmt::Display for QualifierValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifierValue(...)")
    }
}

