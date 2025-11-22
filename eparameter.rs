// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EParameter (struct)
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

use crate::eannotation::EAnnotation;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EParameter {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ordered: Option<bool>,
    unique: Option<bool>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    e_type: Option<String>,
}

#[wasm_bindgen]
impl EParameter {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ordered: None,
            unique: None,
            lower_bound: None,
            upper_bound: None,
            e_type: None,
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

    /// Returns a clone of ordered if present
    pub fn ordered(&self) -> Option<bool> {
        self.ordered.clone()
    }

    /// Sets ordered
    pub fn set_ordered(&mut self, value: bool) {
        self.ordered = Some(value);
    }

    /// Takes ordered, leaving None in its place
    pub fn take_ordered(&mut self) -> Option<bool> {
        self.ordered.take()
    }

    /// Returns a clone of unique if present
    pub fn unique(&self) -> Option<bool> {
        self.unique.clone()
    }

    /// Sets unique
    pub fn set_unique(&mut self, value: bool) {
        self.unique = Some(value);
    }

    /// Takes unique, leaving None in its place
    pub fn take_unique(&mut self) -> Option<bool> {
        self.unique.take()
    }

    /// Returns a clone of lower_bound if present
    pub fn lower_bound(&self) -> Option<i32> {
        self.lower_bound.clone()
    }

    /// Sets lower_bound
    pub fn set_lower_bound(&mut self, value: i32) {
        self.lower_bound = Some(value);
    }

    /// Takes lower_bound, leaving None in its place
    pub fn take_lower_bound(&mut self) -> Option<i32> {
        self.lower_bound.take()
    }

    /// Returns a clone of upper_bound if present
    pub fn upper_bound(&self) -> Option<i32> {
        self.upper_bound.clone()
    }

    /// Sets upper_bound
    pub fn set_upper_bound(&mut self, value: i32) {
        self.upper_bound = Some(value);
    }

    /// Takes upper_bound, leaving None in its place
    pub fn take_upper_bound(&mut self) -> Option<i32> {
        self.upper_bound.take()
    }

    /// Returns a clone of e_type if present
    pub fn e_type(&self) -> Option<String> {
        self.e_type.clone()
    }

    /// Sets e_type
    pub fn set_e_type(&mut self, value: String) {
        self.e_type = Some(value);
    }

    /// Takes e_type, leaving None in its place
    pub fn take_e_type(&mut self) -> Option<String> {
        self.e_type.take()
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
        "EParameter".to_string()
    }

}

impl std::fmt::Display for EParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

