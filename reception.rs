// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Reception (struct)
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
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::parameter::Parameter;
use crate::parameter_set::ParameterSet;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Reception {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
    is_leaf: bool,
    is_static: bool,
    owned_parameter: Vec<Parameter>,
    is_abstract: bool,
    method: Vec<String>,
    concurrency: String,
    raised_exception: Vec<String>,
    owned_parameter_set: Vec<ParameterSet>,
    signal: Option<String>,
}

#[wasm_bindgen]
impl Reception {
    pub fn new(is_leaf: bool, is_static: bool, is_abstract: bool, concurrency: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            is_leaf: is_leaf,
            is_static: is_static,
            owned_parameter: Vec::new(),
            is_abstract: is_abstract,
            method: Vec::new(),
            concurrency: concurrency,
            raised_exception: Vec::new(),
            owned_parameter_set: Vec::new(),
            signal: None,
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

    /// Returns is_abstract
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    /// Sets is_abstract
    pub fn set_is_abstract(&mut self, value: bool) {
        self.is_abstract = value;
    }

    /// Returns a clone of method
    pub fn method(&self) -> Vec<String> {
        self.method.clone()
    }

    /// Adds an existing Behavior to method by ID
    pub fn add_method_by_id(&mut self, id: String) {
        self.method.push(id);
    }

    /// Clears all items from method
    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    /// Returns a clone of concurrency
    pub fn concurrency(&self) -> String {
        self.concurrency.clone()
    }

    /// Sets concurrency
    pub fn set_concurrency(&mut self, value: String) {
        self.concurrency = value;
    }

    /// Takes ownership of concurrency, replacing it with an empty string
    pub fn take_concurrency(&mut self) -> String {
        std::mem::take(&mut self.concurrency)
    }

    /// Returns a clone of raised_exception
    pub fn raised_exception(&self) -> Vec<String> {
        self.raised_exception.clone()
    }

    /// Adds an existing Type to raised_exception by ID
    pub fn add_raised_exception_by_id(&mut self, id: String) {
        self.raised_exception.push(id);
    }

    /// Clears all items from raised_exception
    pub fn clear_raised_exception(&mut self) {
        self.raised_exception.clear();
    }

    /// Returns a clone of signal if present
    pub fn signal(&self) -> Option<String> {
        self.signal.clone()
    }

    /// Sets signal
    pub fn set_signal(&mut self, value: String) {
        self.signal = Some(value);
    }

    /// Takes signal, leaving None in its place
    pub fn take_signal(&mut self) -> Option<String> {
        self.signal.take()
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
        "Reception".to_string()
    }

}

impl std::fmt::Display for Reception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

