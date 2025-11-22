// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ProtocolConformance (struct)
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
pub struct ProtocolConformance {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    general_machine: String,
    specific_machine: String,
}

#[wasm_bindgen]
impl ProtocolConformance {
    pub fn new(general_machine: String, specific_machine: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            general_machine: general_machine,
            specific_machine: specific_machine,
        }
    }

    /// Returns a clone of general_machine
    pub fn general_machine(&self) -> String {
        self.general_machine.clone()
    }

    /// Sets general_machine
    pub fn set_general_machine(&mut self, value: String) {
        self.general_machine = value;
    }

    /// Takes ownership of general_machine, replacing it with an empty string
    pub fn take_general_machine(&mut self) -> String {
        std::mem::take(&mut self.general_machine)
    }

    /// Returns a clone of specific_machine
    pub fn specific_machine(&self) -> String {
        self.specific_machine.clone()
    }

    /// Sets specific_machine
    pub fn set_specific_machine(&mut self, value: String) {
        self.specific_machine = value;
    }

    /// Takes ownership of specific_machine, replacing it with an empty string
    pub fn take_specific_machine(&mut self) -> String {
        std::mem::take(&mut self.specific_machine)
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
        "ProtocolConformance".to_string()
    }

}

impl Default for ProtocolConformance {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            general_machine: Default::default(),
            specific_machine: Default::default(),
        }
    }
}

impl std::fmt::Display for ProtocolConformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtocolConformance(...)")
    }
}

