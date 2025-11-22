// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           TemplateParameterSubstitution (struct)
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
use crate::parameterable_element::ParameterableElement;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TemplateParameterSubstitution {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    formal: String,
    actual: Vec<String>,
    owned_actual: Vec<ParameterableElement>,
    template_binding: String,
}

#[wasm_bindgen]
impl TemplateParameterSubstitution {
    pub fn new(formal: String, actual: Vec<String>, template_binding: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            formal: formal,
            actual: actual,
            owned_actual: Vec::new(),
            template_binding: template_binding,
        }
    }

    /// Returns a clone of formal
    pub fn formal(&self) -> String {
        self.formal.clone()
    }

    /// Sets formal
    pub fn set_formal(&mut self, value: String) {
        self.formal = value;
    }

    /// Takes ownership of formal, replacing it with an empty string
    pub fn take_formal(&mut self) -> String {
        std::mem::take(&mut self.formal)
    }

    /// Returns a clone of actual
    pub fn actual(&self) -> Vec<String> {
        self.actual.clone()
    }

    /// Adds an existing ParameterableElement to actual by ID
    pub fn add_actual_by_id(&mut self, id: String) {
        self.actual.push(id);
    }

    /// Clears all items from actual
    pub fn clear_actual(&mut self) {
        self.actual.clear();
    }

    /// Returns a clone of template_binding
    pub fn template_binding(&self) -> String {
        self.template_binding.clone()
    }

    /// Sets template_binding
    pub fn set_template_binding(&mut self, value: String) {
        self.template_binding = value;
    }

    /// Takes ownership of template_binding, replacing it with an empty string
    pub fn take_template_binding(&mut self) -> String {
        std::mem::take(&mut self.template_binding)
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
        "TemplateParameterSubstitution".to_string()
    }

}

impl Default for TemplateParameterSubstitution {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            formal: Default::default(),
            actual: Vec::new(),
            owned_actual: Vec::new(),
            template_binding: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateParameterSubstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateParameterSubstitution(...)")
    }
}

