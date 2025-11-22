// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ProtocolTransition (struct)
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
use crate::behavior::Behavior;
use crate::trigger::Trigger;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ProtocolTransition {
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
    kind: String,
    container: String,
    redefined_transition: Option<String>,
    guard: Option<String>,
    effect: Option<Behavior>,
    trigger: Vec<Trigger>,
    target: String,
    source: String,
    post_condition: Option<String>,
    pre_condition: Option<String>,
}

#[wasm_bindgen]
impl ProtocolTransition {
    pub fn new(is_leaf: bool, kind: String, container: String, target: String, source: String) -> Self {
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
            kind: kind,
            container: container,
            redefined_transition: None,
            guard: None,
            effect: None,
            trigger: Vec::new(),
            target: target,
            source: source,
            post_condition: None,
            pre_condition: None,
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

    /// Returns a clone of kind
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    /// Sets kind
    pub fn set_kind(&mut self, value: String) {
        self.kind = value;
    }

    /// Takes ownership of kind, replacing it with an empty string
    pub fn take_kind(&mut self) -> String {
        std::mem::take(&mut self.kind)
    }

    /// Returns a clone of container
    pub fn container(&self) -> String {
        self.container.clone()
    }

    /// Sets container
    pub fn set_container(&mut self, value: String) {
        self.container = value;
    }

    /// Takes ownership of container, replacing it with an empty string
    pub fn take_container(&mut self) -> String {
        std::mem::take(&mut self.container)
    }

    /// Returns a clone of redefined_transition if present
    pub fn redefined_transition(&self) -> Option<String> {
        self.redefined_transition.clone()
    }

    /// Sets redefined_transition
    pub fn set_redefined_transition(&mut self, value: String) {
        self.redefined_transition = Some(value);
    }

    /// Takes redefined_transition, leaving None in its place
    pub fn take_redefined_transition(&mut self) -> Option<String> {
        self.redefined_transition.take()
    }

    /// Returns a clone of guard if present
    pub fn guard(&self) -> Option<String> {
        self.guard.clone()
    }

    /// Sets guard
    pub fn set_guard(&mut self, value: String) {
        self.guard = Some(value);
    }

    /// Takes guard, leaving None in its place
    pub fn take_guard(&mut self) -> Option<String> {
        self.guard.take()
    }

    /// Returns a clone of target
    pub fn target(&self) -> String {
        self.target.clone()
    }

    /// Sets target
    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }

    /// Takes ownership of target, replacing it with an empty string
    pub fn take_target(&mut self) -> String {
        std::mem::take(&mut self.target)
    }

    /// Returns a clone of source
    pub fn source(&self) -> String {
        self.source.clone()
    }

    /// Sets source
    pub fn set_source(&mut self, value: String) {
        self.source = value;
    }

    /// Takes ownership of source, replacing it with an empty string
    pub fn take_source(&mut self) -> String {
        std::mem::take(&mut self.source)
    }

    /// Returns a clone of post_condition if present
    pub fn post_condition(&self) -> Option<String> {
        self.post_condition.clone()
    }

    /// Sets post_condition
    pub fn set_post_condition(&mut self, value: String) {
        self.post_condition = Some(value);
    }

    /// Takes post_condition, leaving None in its place
    pub fn take_post_condition(&mut self) -> Option<String> {
        self.post_condition.take()
    }

    /// Returns a clone of pre_condition if present
    pub fn pre_condition(&self) -> Option<String> {
        self.pre_condition.clone()
    }

    /// Sets pre_condition
    pub fn set_pre_condition(&mut self, value: String) {
        self.pre_condition = Some(value);
    }

    /// Takes pre_condition, leaving None in its place
    pub fn take_pre_condition(&mut self) -> Option<String> {
        self.pre_condition.take()
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
        "ProtocolTransition".to_string()
    }

}

impl Default for ProtocolTransition {
    fn default() -> Self {
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
            is_leaf: false,
            kind: String::new(),
            container: Default::default(),
            redefined_transition: None,
            guard: None,
            effect: None,
            trigger: Vec::new(),
            target: Default::default(),
            source: Default::default(),
            post_condition: None,
            pre_condition: None,
        }
    }
}

impl std::fmt::Display for ProtocolTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

