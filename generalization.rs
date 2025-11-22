// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Generalization (struct)
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
use crate::comment::Comment;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Generalization {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    is_substitutable: Option<bool>,
    general: String,
    generalization_set: Vec<String>,
    specific: String,
}

#[wasm_bindgen]
impl Generalization {
    pub fn new(general: String, specific: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_substitutable: None,
            general: general,
            generalization_set: Vec::new(),
            specific: specific,
        }
    }

    /// Returns a clone of is_substitutable if present
    pub fn is_substitutable(&self) -> Option<bool> {
        self.is_substitutable.clone()
    }

    /// Sets is_substitutable
    pub fn set_is_substitutable(&mut self, value: bool) {
        self.is_substitutable = Some(value);
    }

    /// Takes is_substitutable, leaving None in its place
    pub fn take_is_substitutable(&mut self) -> Option<bool> {
        self.is_substitutable.take()
    }

    /// Returns a clone of general
    pub fn general(&self) -> String {
        self.general.clone()
    }

    /// Sets general
    pub fn set_general(&mut self, value: String) {
        self.general = value;
    }

    /// Takes ownership of general, replacing it with an empty string
    pub fn take_general(&mut self) -> String {
        std::mem::take(&mut self.general)
    }

    /// Returns a clone of generalization_set
    pub fn generalization_set(&self) -> Vec<String> {
        self.generalization_set.clone()
    }

    /// Adds an existing GeneralizationSet to generalization_set by ID
    pub fn add_generalization_set_by_id(&mut self, id: String) {
        self.generalization_set.push(id);
    }

    /// Clears all items from generalization_set
    pub fn clear_generalization_set(&mut self) {
        self.generalization_set.clear();
    }

    /// Returns a clone of specific
    pub fn specific(&self) -> String {
        self.specific.clone()
    }

    /// Sets specific
    pub fn set_specific(&mut self, value: String) {
        self.specific = value;
    }

    /// Takes ownership of specific, replacing it with an empty string
    pub fn take_specific(&mut self) -> String {
        std::mem::take(&mut self.specific)
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
        "Generalization".to_string()
    }

}

impl Default for Generalization {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_substitutable: None,
            general: Default::default(),
            generalization_set: Vec::new(),
            specific: Default::default(),
        }
    }
}

impl std::fmt::Display for Generalization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generalization(...)")
    }
}

