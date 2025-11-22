// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           TemplateBinding (struct)
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
use crate::template_parameter_substitution::TemplateParameterSubstitution;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TemplateBinding {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    signature: String,
    parameter_substitution: Vec<TemplateParameterSubstitution>,
    bound_element: String,
}

#[wasm_bindgen]
impl TemplateBinding {
    pub fn new(signature: String, bound_element: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: signature,
            parameter_substitution: Vec::new(),
            bound_element: bound_element,
        }
    }

    /// Returns a clone of signature
    pub fn signature(&self) -> String {
        self.signature.clone()
    }

    /// Sets signature
    pub fn set_signature(&mut self, value: String) {
        self.signature = value;
    }

    /// Takes ownership of signature, replacing it with an empty string
    pub fn take_signature(&mut self) -> String {
        std::mem::take(&mut self.signature)
    }

    /// Returns a clone of bound_element
    pub fn bound_element(&self) -> String {
        self.bound_element.clone()
    }

    /// Sets bound_element
    pub fn set_bound_element(&mut self, value: String) {
        self.bound_element = value;
    }

    /// Takes ownership of bound_element, replacing it with an empty string
    pub fn take_bound_element(&mut self) -> String {
        std::mem::take(&mut self.bound_element)
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
        "TemplateBinding".to_string()
    }

}

impl Default for TemplateBinding {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: Default::default(),
            parameter_substitution: Vec::new(),
            bound_element: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateBinding(...)")
    }
}

