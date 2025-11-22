// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           LinkEndDestructionData (struct)
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
use crate::qualifier_value::QualifierValue;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LinkEndDestructionData {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    value: Option<String>,
    end: String,
    qualifier: Vec<QualifierValue>,
    is_destroy_duplicates: bool,
    destroy_at: Option<String>,
}

#[wasm_bindgen]
impl LinkEndDestructionData {
    pub fn new(end: String, is_destroy_duplicates: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            value: None,
            end: end,
            qualifier: Vec::new(),
            is_destroy_duplicates: is_destroy_duplicates,
            destroy_at: None,
        }
    }

    /// Returns a clone of value if present
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }

    /// Sets value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Takes value, leaving None in its place
    pub fn take_value(&mut self) -> Option<String> {
        self.value.take()
    }

    /// Returns a clone of end
    pub fn end(&self) -> String {
        self.end.clone()
    }

    /// Sets end
    pub fn set_end(&mut self, value: String) {
        self.end = value;
    }

    /// Takes ownership of end, replacing it with an empty string
    pub fn take_end(&mut self) -> String {
        std::mem::take(&mut self.end)
    }

    /// Returns is_destroy_duplicates
    pub fn is_destroy_duplicates(&self) -> bool {
        self.is_destroy_duplicates
    }

    /// Sets is_destroy_duplicates
    pub fn set_is_destroy_duplicates(&mut self, value: bool) {
        self.is_destroy_duplicates = value;
    }

    /// Returns a clone of destroy_at if present
    pub fn destroy_at(&self) -> Option<String> {
        self.destroy_at.clone()
    }

    /// Sets destroy_at
    pub fn set_destroy_at(&mut self, value: String) {
        self.destroy_at = Some(value);
    }

    /// Takes destroy_at, leaving None in its place
    pub fn take_destroy_at(&mut self) -> Option<String> {
        self.destroy_at.take()
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
        "LinkEndDestructionData".to_string()
    }

}

impl Default for LinkEndDestructionData {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            value: None,
            end: Default::default(),
            qualifier: Vec::new(),
            is_destroy_duplicates: false,
            destroy_at: None,
        }
    }
}

impl std::fmt::Display for LinkEndDestructionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkEndDestructionData(...)")
    }
}

