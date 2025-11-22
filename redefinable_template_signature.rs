// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           RedefinableTemplateSignature (struct)
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
use crate::string_expression::StringExpression;
use crate::template_parameter::TemplateParameter;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct RedefinableTemplateSignature {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    parameter: Vec<String>,
    owned_parameter: Vec<TemplateParameter>,
    template: String,
    extended_signature: Vec<String>,
}

#[wasm_bindgen]
impl RedefinableTemplateSignature {
    pub fn new(is_leaf: bool, parameter: Vec<String>, template: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            parameter: parameter,
            owned_parameter: Vec::new(),
            template: template,
            extended_signature: Vec::new(),
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

    /// Returns a clone of visibility if present
    pub fn visibility(&self) -> Option<String> {
        self.visibility.clone()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = Some(value);
    }

    /// Takes visibility, leaving None in its place
    pub fn take_visibility(&mut self) -> Option<String> {
        self.visibility.take()
    }

    /// Returns a clone of client_dependency
    pub fn client_dependency(&self) -> Vec<String> {
        self.client_dependency.clone()
    }

    /// Adds an existing Dependency to client_dependency by ID
    pub fn add_client_dependency_by_id(&mut self, id: String) {
        self.client_dependency.push(id);
    }

    /// Clears all items from client_dependency
    pub fn clear_client_dependency(&mut self) {
        self.client_dependency.clear();
    }

    /// Returns is_leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Sets is_leaf
    pub fn set_is_leaf(&mut self, value: bool) {
        self.is_leaf = value;
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

    /// Returns a clone of extended_signature
    pub fn extended_signature(&self) -> Vec<String> {
        self.extended_signature.clone()
    }

    /// Adds an existing RedefinableTemplateSignature to extended_signature by ID
    pub fn add_extended_signature_by_id(&mut self, id: String) {
        self.extended_signature.push(id);
    }

    /// Clears all items from extended_signature
    pub fn clear_extended_signature(&mut self) {
        self.extended_signature.clear();
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
        "RedefinableTemplateSignature".to_string()
    }

}

impl Default for RedefinableTemplateSignature {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            parameter: Vec::new(),
            owned_parameter: Vec::new(),
            template: Default::default(),
            extended_signature: Vec::new(),
        }
    }
}

impl std::fmt::Display for RedefinableTemplateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

