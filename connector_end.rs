// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ConnectorEnd (struct)
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
use crate::value_specification::ValueSpecification;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ConnectorEnd {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    part_with_port: Option<String>,
    role: String,
}

#[wasm_bindgen]
impl ConnectorEnd {
    pub fn new(is_ordered: bool, is_unique: bool, role: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            part_with_port: None,
            role: role,
        }
    }

    /// Returns is_ordered
    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    /// Sets is_ordered
    pub fn set_is_ordered(&mut self, value: bool) {
        self.is_ordered = value;
    }

    /// Returns is_unique
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }

    /// Sets is_unique
    pub fn set_is_unique(&mut self, value: bool) {
        self.is_unique = value;
    }

    /// Returns a clone of part_with_port if present
    pub fn part_with_port(&self) -> Option<String> {
        self.part_with_port.clone()
    }

    /// Sets part_with_port
    pub fn set_part_with_port(&mut self, value: String) {
        self.part_with_port = Some(value);
    }

    /// Takes part_with_port, leaving None in its place
    pub fn take_part_with_port(&mut self) -> Option<String> {
        self.part_with_port.take()
    }

    /// Returns a clone of role
    pub fn role(&self) -> String {
        self.role.clone()
    }

    /// Sets role
    pub fn set_role(&mut self, value: String) {
        self.role = value;
    }

    /// Takes ownership of role, replacing it with an empty string
    pub fn take_role(&mut self) -> String {
        std::mem::take(&mut self.role)
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
        false
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        true
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "ConnectorEnd".to_string()
    }

}

impl Default for ConnectorEnd {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_ordered: false,
            is_unique: false,
            upper_value: None,
            lower_value: None,
            part_with_port: None,
            role: Default::default(),
        }
    }
}

impl std::fmt::Display for ConnectorEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectorEnd(...)")
    }
}

