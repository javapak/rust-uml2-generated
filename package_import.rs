// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           PackageImport (struct)
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
pub struct PackageImport {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    visibility: String,
    imported_package: String,
    importing_namespace: String,
}

#[wasm_bindgen]
impl PackageImport {
    pub fn new(visibility: String, imported_package: String, importing_namespace: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            visibility: visibility,
            imported_package: imported_package,
            importing_namespace: importing_namespace,
        }
    }

    /// Returns a clone of visibility
    pub fn visibility(&self) -> String {
        self.visibility.clone()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = value;
    }

    /// Takes ownership of visibility, replacing it with an empty string
    pub fn take_visibility(&mut self) -> String {
        std::mem::take(&mut self.visibility)
    }

    /// Returns a clone of imported_package
    pub fn imported_package(&self) -> String {
        self.imported_package.clone()
    }

    /// Sets imported_package
    pub fn set_imported_package(&mut self, value: String) {
        self.imported_package = value;
    }

    /// Takes ownership of imported_package, replacing it with an empty string
    pub fn take_imported_package(&mut self) -> String {
        std::mem::take(&mut self.imported_package)
    }

    /// Returns a clone of importing_namespace
    pub fn importing_namespace(&self) -> String {
        self.importing_namespace.clone()
    }

    /// Sets importing_namespace
    pub fn set_importing_namespace(&mut self, value: String) {
        self.importing_namespace = value;
    }

    /// Takes ownership of importing_namespace, replacing it with an empty string
    pub fn take_importing_namespace(&mut self) -> String {
        std::mem::take(&mut self.importing_namespace)
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
        "PackageImport".to_string()
    }

}

impl Default for PackageImport {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            visibility: String::new(),
            imported_package: Default::default(),
            importing_namespace: Default::default(),
        }
    }
}

impl std::fmt::Display for PackageImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageImport(...)")
    }
}

