// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ComponentRealization (struct)
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
use crate::opaque_expression::OpaqueExpression;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ComponentRealization {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    supplier: Vec<String>,
    client: Vec<String>,
    mapping: Option<OpaqueExpression>,
    abstraction: Option<String>,
    realizing_classifier: String,
}

#[wasm_bindgen]
impl ComponentRealization {
    pub fn new(supplier: Vec<String>, client: Vec<String>, realizing_classifier: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            owning_template_parameter: None,
            template_parameter: None,
            supplier: supplier,
            client: client,
            mapping: None,
            abstraction: None,
            realizing_classifier: realizing_classifier,
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

    /// Returns a clone of owning_template_parameter if present
    pub fn owning_template_parameter(&self) -> Option<String> {
        self.owning_template_parameter.clone()
    }

    /// Sets owning_template_parameter
    pub fn set_owning_template_parameter(&mut self, value: String) {
        self.owning_template_parameter = Some(value);
    }

    /// Takes owning_template_parameter, leaving None in its place
    pub fn take_owning_template_parameter(&mut self) -> Option<String> {
        self.owning_template_parameter.take()
    }

    /// Returns a clone of template_parameter if present
    pub fn template_parameter(&self) -> Option<String> {
        self.template_parameter.clone()
    }

    /// Sets template_parameter
    pub fn set_template_parameter(&mut self, value: String) {
        self.template_parameter = Some(value);
    }

    /// Takes template_parameter, leaving None in its place
    pub fn take_template_parameter(&mut self) -> Option<String> {
        self.template_parameter.take()
    }

    /// Returns a clone of supplier
    pub fn supplier(&self) -> Vec<String> {
        self.supplier.clone()
    }

    /// Adds an existing NamedElement to supplier by ID
    pub fn add_supplier_by_id(&mut self, id: String) {
        self.supplier.push(id);
    }

    /// Clears all items from supplier
    pub fn clear_supplier(&mut self) {
        self.supplier.clear();
    }

    /// Returns a clone of client
    pub fn client(&self) -> Vec<String> {
        self.client.clone()
    }

    /// Adds an existing NamedElement to client by ID
    pub fn add_client_by_id(&mut self, id: String) {
        self.client.push(id);
    }

    /// Clears all items from client
    pub fn clear_client(&mut self) {
        self.client.clear();
    }

    /// Returns a clone of abstraction if present
    pub fn abstraction(&self) -> Option<String> {
        self.abstraction.clone()
    }

    /// Sets abstraction
    pub fn set_abstraction(&mut self, value: String) {
        self.abstraction = Some(value);
    }

    /// Takes abstraction, leaving None in its place
    pub fn take_abstraction(&mut self) -> Option<String> {
        self.abstraction.take()
    }

    /// Returns a clone of realizing_classifier
    pub fn realizing_classifier(&self) -> String {
        self.realizing_classifier.clone()
    }

    /// Sets realizing_classifier
    pub fn set_realizing_classifier(&mut self, value: String) {
        self.realizing_classifier = value;
    }

    /// Takes ownership of realizing_classifier, replacing it with an empty string
    pub fn take_realizing_classifier(&mut self) -> String {
        std::mem::take(&mut self.realizing_classifier)
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
        "ComponentRealization".to_string()
    }

}

impl Default for ComponentRealization {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            owning_template_parameter: None,
            template_parameter: None,
            supplier: Vec::new(),
            client: Vec::new(),
            mapping: None,
            abstraction: None,
            realizing_classifier: Default::default(),
        }
    }
}

impl std::fmt::Display for ComponentRealization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

