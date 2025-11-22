// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Slot (struct)
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
use crate::value_specification::ValueSpecification;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Slot {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    defining_feature: String,
    value: Vec<ValueSpecification>,
    owning_instance: String,
}

#[wasm_bindgen]
impl Slot {
    pub fn new(defining_feature: String, owning_instance: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            defining_feature: defining_feature,
            value: Vec::new(),
            owning_instance: owning_instance,
        }
    }

    /// Returns a clone of defining_feature
    pub fn defining_feature(&self) -> String {
        self.defining_feature.clone()
    }

    /// Sets defining_feature
    pub fn set_defining_feature(&mut self, value: String) {
        self.defining_feature = value;
    }

    /// Takes ownership of defining_feature, replacing it with an empty string
    pub fn take_defining_feature(&mut self) -> String {
        std::mem::take(&mut self.defining_feature)
    }

    /// Returns a clone of owning_instance
    pub fn owning_instance(&self) -> String {
        self.owning_instance.clone()
    }

    /// Sets owning_instance
    pub fn set_owning_instance(&mut self, value: String) {
        self.owning_instance = value;
    }

    /// Takes ownership of owning_instance, replacing it with an empty string
    pub fn take_owning_instance(&mut self) -> String {
        std::mem::take(&mut self.owning_instance)
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
        "Slot".to_string()
    }

}

impl Default for Slot {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            defining_feature: Default::default(),
            value: Vec::new(),
            owning_instance: Default::default(),
        }
    }
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot(...)")
    }
}

