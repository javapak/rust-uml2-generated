// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           PackageMerge (struct)
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
pub struct PackageMerge {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    merged_package: String,
    receiving_package: String,
}

#[wasm_bindgen]
impl PackageMerge {
    pub fn new(merged_package: String, receiving_package: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            merged_package: merged_package,
            receiving_package: receiving_package,
        }
    }

    /// Returns a clone of merged_package
    pub fn merged_package(&self) -> String {
        self.merged_package.clone()
    }

    /// Sets merged_package
    pub fn set_merged_package(&mut self, value: String) {
        self.merged_package = value;
    }

    /// Takes ownership of merged_package, replacing it with an empty string
    pub fn take_merged_package(&mut self) -> String {
        std::mem::take(&mut self.merged_package)
    }

    /// Returns a clone of receiving_package
    pub fn receiving_package(&self) -> String {
        self.receiving_package.clone()
    }

    /// Sets receiving_package
    pub fn set_receiving_package(&mut self, value: String) {
        self.receiving_package = value;
    }

    /// Takes ownership of receiving_package, replacing it with an empty string
    pub fn take_receiving_package(&mut self) -> String {
        std::mem::take(&mut self.receiving_package)
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
        "PackageMerge".to_string()
    }

}

impl Default for PackageMerge {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            merged_package: Default::default(),
            receiving_package: Default::default(),
        }
    }
}

impl std::fmt::Display for PackageMerge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageMerge(...)")
    }
}

