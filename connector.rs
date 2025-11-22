// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Connector (struct)
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
use crate::connector_end::ConnectorEnd;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Connector {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    is_static: bool,
    type_: Option<String>,
    redefined_connector: Vec<String>,
    end: Vec<ConnectorEnd>,
    kind: Option<String>,
    contract: Vec<String>,
}

#[wasm_bindgen]
impl Connector {
    pub fn new(is_leaf: bool, is_static: bool, end: Vec<ConnectorEnd>) -> Self {
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
            redefined_connector: Vec::new(),
            end: end,
            kind: None,
            contract: Vec::new(),
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

    /// Returns a clone of redefined_connector
    pub fn redefined_connector(&self) -> Vec<String> {
        self.redefined_connector.clone()
    }

    /// Adds an existing Connector to redefined_connector by ID
    pub fn add_redefined_connector_by_id(&mut self, id: String) {
        self.redefined_connector.push(id);
    }

    /// Clears all items from redefined_connector
    pub fn clear_redefined_connector(&mut self) {
        self.redefined_connector.clear();
    }

    /// Returns a clone of kind if present
    pub fn kind(&self) -> Option<String> {
        self.kind.clone()
    }

    /// Sets kind
    pub fn set_kind(&mut self, value: String) {
        self.kind = Some(value);
    }

    /// Takes kind, leaving None in its place
    pub fn take_kind(&mut self) -> Option<String> {
        self.kind.take()
    }

    /// Returns a clone of contract
    pub fn contract(&self) -> Vec<String> {
        self.contract.clone()
    }

    /// Adds an existing Behavior to contract by ID
    pub fn add_contract_by_id(&mut self, id: String) {
        self.contract.push(id);
    }

    /// Clears all items from contract
    pub fn clear_contract(&mut self) {
        self.contract.clear();
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
        "Connector".to_string()
    }

}

impl std::fmt::Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

