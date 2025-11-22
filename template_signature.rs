// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           TemplateSignature (struct)
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
use crate::template_parameter::TemplateParameter;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TemplateSignature {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    parameter: Vec<String>,
    owned_parameter: Vec<TemplateParameter>,
    template: String,
}

#[wasm_bindgen]
impl TemplateSignature {
    pub fn new(parameter: Vec<String>, template: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            parameter: parameter,
            owned_parameter: Vec::new(),
            template: template,
        }
    }

    /// Returns a clone of parameter
    pub fn parameter(&self) -> Vec<String> {
        self.parameter.clone()
    }

    /// Adds an existing TemplateParameter to parameter by ID
    pub fn add_parameter_by_id(&mut self, id: String) {
        self.parameter.push(id);
    }

    /// Clears all items from parameter
    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    /// Returns a clone of template
    pub fn template(&self) -> String {
        self.template.clone()
    }

    /// Sets template
    pub fn set_template(&mut self, value: String) {
        self.template = value;
    }

    /// Takes ownership of template, replacing it with an empty string
    pub fn take_template(&mut self) -> String {
        std::mem::take(&mut self.template)
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
        "TemplateSignature".to_string()
    }

}

impl Default for TemplateSignature {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            parameter: Vec::new(),
            owned_parameter: Vec::new(),
            template: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateSignature(...)")
    }
}

