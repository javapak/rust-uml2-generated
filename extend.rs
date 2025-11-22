// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Extend (struct)
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
use crate::constraint::Constraint;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Extend {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    extended_case: String,
    condition: Option<Constraint>,
    extension_location: Vec<String>,
    extension: String,
}

#[wasm_bindgen]
impl Extend {
    pub fn new(extended_case: String, extension_location: Vec<String>, extension: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            extended_case: extended_case,
            condition: None,
            extension_location: extension_location,
            extension: extension,
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

    /// Returns a clone of extended_case
    pub fn extended_case(&self) -> String {
        self.extended_case.clone()
    }

    /// Sets extended_case
    pub fn set_extended_case(&mut self, value: String) {
        self.extended_case = value;
    }

    /// Takes ownership of extended_case, replacing it with an empty string
    pub fn take_extended_case(&mut self) -> String {
        std::mem::take(&mut self.extended_case)
    }

    /// Returns a clone of extension_location
    pub fn extension_location(&self) -> Vec<String> {
        self.extension_location.clone()
    }

    /// Adds an existing ExtensionPoint to extension_location by ID
    pub fn add_extension_location_by_id(&mut self, id: String) {
        self.extension_location.push(id);
    }

    /// Clears all items from extension_location
    pub fn clear_extension_location(&mut self) {
        self.extension_location.clear();
    }

    /// Returns a clone of extension
    pub fn extension(&self) -> String {
        self.extension.clone()
    }

    /// Sets extension
    pub fn set_extension(&mut self, value: String) {
        self.extension = value;
    }

    /// Takes ownership of extension, replacing it with an empty string
    pub fn take_extension(&mut self) -> String {
        std::mem::take(&mut self.extension)
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
        "Extend".to_string()
    }

}

impl Default for Extend {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            extended_case: Default::default(),
            condition: None,
            extension_location: Vec::new(),
            extension: Default::default(),
        }
    }
}

impl std::fmt::Display for Extend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

