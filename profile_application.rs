// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ProfileApplication (struct)
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
pub struct ProfileApplication {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    applied_profile: String,
    is_strict: bool,
    applying_package: String,
}

#[wasm_bindgen]
impl ProfileApplication {
    pub fn new(applied_profile: String, is_strict: bool, applying_package: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            applied_profile: applied_profile,
            is_strict: is_strict,
            applying_package: applying_package,
        }
    }

    /// Returns a clone of applied_profile
    pub fn applied_profile(&self) -> String {
        self.applied_profile.clone()
    }

    /// Sets applied_profile
    pub fn set_applied_profile(&mut self, value: String) {
        self.applied_profile = value;
    }

    /// Takes ownership of applied_profile, replacing it with an empty string
    pub fn take_applied_profile(&mut self) -> String {
        std::mem::take(&mut self.applied_profile)
    }

    /// Returns is_strict
    pub fn is_strict(&self) -> bool {
        self.is_strict
    }

    /// Sets is_strict
    pub fn set_is_strict(&mut self, value: bool) {
        self.is_strict = value;
    }

    /// Returns a clone of applying_package
    pub fn applying_package(&self) -> String {
        self.applying_package.clone()
    }

    /// Sets applying_package
    pub fn set_applying_package(&mut self, value: String) {
        self.applying_package = value;
    }

    /// Takes ownership of applying_package, replacing it with an empty string
    pub fn take_applying_package(&mut self) -> String {
        std::mem::take(&mut self.applying_package)
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
        "ProfileApplication".to_string()
    }

}

impl Default for ProfileApplication {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            applied_profile: Default::default(),
            is_strict: false,
            applying_package: Default::default(),
        }
    }
}

impl std::fmt::Display for ProfileApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProfileApplication(...)")
    }
}

