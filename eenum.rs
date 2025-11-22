// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EEnum (struct)
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
use crate::eenum_literal::EEnumLiteral;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EEnum {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    instance_class_name: Option<String>,
    instance_type_name: Option<String>,
    serializable: Option<bool>,
    e_literals: Vec<EEnumLiteral>,
}

#[wasm_bindgen]
impl EEnum {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            instance_class_name: None,
            instance_type_name: None,
            serializable: None,
            e_literals: Vec::new(),
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

    /// Returns a clone of instance_class_name if present
    pub fn instance_class_name(&self) -> Option<String> {
        self.instance_class_name.clone()
    }

    /// Sets instance_class_name
    pub fn set_instance_class_name(&mut self, value: String) {
        self.instance_class_name = Some(value);
    }

    /// Takes instance_class_name, leaving None in its place
    pub fn take_instance_class_name(&mut self) -> Option<String> {
        self.instance_class_name.take()
    }

    /// Returns a clone of instance_type_name if present
    pub fn instance_type_name(&self) -> Option<String> {
        self.instance_type_name.clone()
    }

    /// Sets instance_type_name
    pub fn set_instance_type_name(&mut self, value: String) {
        self.instance_type_name = Some(value);
    }

    /// Takes instance_type_name, leaving None in its place
    pub fn take_instance_type_name(&mut self) -> Option<String> {
        self.instance_type_name.take()
    }

    /// Returns a clone of serializable if present
    pub fn serializable(&self) -> Option<bool> {
        self.serializable.clone()
    }

    /// Sets serializable
    pub fn set_serializable(&mut self, value: bool) {
        self.serializable = Some(value);
    }

    /// Takes serializable, leaving None in its place
    pub fn take_serializable(&mut self) -> Option<bool> {
        self.serializable.take()
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
        "EEnum".to_string()
    }

}

impl std::fmt::Display for EEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

