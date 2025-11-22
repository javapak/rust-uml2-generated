// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EEnumLiteral (struct)
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
pub struct EEnumLiteral {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    value: Option<i32>,
    literal: Option<String>,
}

#[wasm_bindgen]
impl EEnumLiteral {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            value: None,
            literal: None,
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

    /// Returns a clone of value if present
    pub fn value(&self) -> Option<i32> {
        self.value.clone()
    }

    /// Sets value
    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    /// Takes value, leaving None in its place
    pub fn take_value(&mut self) -> Option<i32> {
        self.value.take()
    }

    /// Returns a clone of literal if present
    pub fn literal(&self) -> Option<String> {
        self.literal.clone()
    }

    /// Sets literal
    pub fn set_literal(&mut self, value: String) {
        self.literal = Some(value);
    }

    /// Takes literal, leaving None in its place
    pub fn take_literal(&mut self) -> Option<String> {
        self.literal.take()
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
        "EEnumLiteral".to_string()
    }

}

impl std::fmt::Display for EEnumLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

