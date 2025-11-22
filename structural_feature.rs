// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           StructuralFeature (struct)
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
use crate::value_specification::ValueSpecification;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct StructuralFeature {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    is_static: bool,
    type_: Option<String>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    is_read_only: bool,
}

#[wasm_bindgen]
impl StructuralFeature {
    pub fn new(is_leaf: bool, is_static: bool, is_ordered: bool, is_unique: bool, is_read_only: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            is_static: is_static,
            type_: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            is_read_only: is_read_only,
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

    /// Returns is_static
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Sets is_static
    pub fn set_is_static(&mut self, value: bool) {
        self.is_static = value;
    }

    /// Returns a clone of type_ if present
    pub fn type_(&self) -> Option<String> {
        self.type_.clone()
    }

    /// Sets type_
    pub fn set_type_(&mut self, value: String) {
        self.type_ = Some(value);
    }

    /// Takes type_, leaving None in its place
    pub fn take_type(&mut self) -> Option<String> {
        self.type_.take()
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

    /// Returns is_read_only
    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    /// Sets is_read_only
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = value;
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
        "StructuralFeature".to_string()
    }

}

