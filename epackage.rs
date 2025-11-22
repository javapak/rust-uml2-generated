// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EPackage (struct)
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
use crate::eclassifier::EClassifier;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EPackage {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ns_uri: Option<String>,
    ns_prefix: Option<String>,
    e_classifiers: Vec<EClassifier>,
    e_subpackages: Vec<Box<EPackage>>,
}

#[wasm_bindgen]
impl EPackage {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ns_uri: None,
            ns_prefix: None,
            e_classifiers: Vec::new(),
            e_subpackages: Vec::new(),
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

    /// Returns a clone of ns_uri if present
    pub fn ns_uri(&self) -> Option<String> {
        self.ns_uri.clone()
    }

    /// Sets ns_uri
    pub fn set_ns_uri(&mut self, value: String) {
        self.ns_uri = Some(value);
    }

    /// Takes ns_uri, leaving None in its place
    pub fn take_ns_uri(&mut self) -> Option<String> {
        self.ns_uri.take()
    }

    /// Returns a clone of ns_prefix if present
    pub fn ns_prefix(&self) -> Option<String> {
        self.ns_prefix.clone()
    }

    /// Sets ns_prefix
    pub fn set_ns_prefix(&mut self, value: String) {
        self.ns_prefix = Some(value);
    }

    /// Takes ns_prefix, leaving None in its place
    pub fn take_ns_prefix(&mut self) -> Option<String> {
        self.ns_prefix.take()
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
        "EPackage".to_string()
    }

}

impl std::fmt::Display for EPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

