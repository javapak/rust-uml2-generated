// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Parameter (struct)
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
use crate::string_expression::StringExpression;
use crate::value_specification::ValueSpecification;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Parameter {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    type_: Option<String>,
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    parameter_set: Vec<String>,
    direction: String,
    default_value: Option<ValueSpecification>,
    is_exception: bool,
    is_stream: bool,
    effect: Option<String>,
}

#[wasm_bindgen]
impl Parameter {
    pub fn new(is_ordered: bool, is_unique: bool, direction: String, is_exception: bool, is_stream: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            type_: None,
            owning_template_parameter: None,
            template_parameter: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            parameter_set: Vec::new(),
            direction: direction,
            default_value: None,
            is_exception: is_exception,
            is_stream: is_stream,
            effect: None,
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

    /// Returns a clone of parameter_set
    pub fn parameter_set(&self) -> Vec<String> {
        self.parameter_set.clone()
    }

    /// Adds an existing ParameterSet to parameter_set by ID
    pub fn add_parameter_set_by_id(&mut self, id: String) {
        self.parameter_set.push(id);
    }

    /// Clears all items from parameter_set
    pub fn clear_parameter_set(&mut self) {
        self.parameter_set.clear();
    }

    /// Returns a clone of direction
    pub fn direction(&self) -> String {
        self.direction.clone()
    }

    /// Sets direction
    pub fn set_direction(&mut self, value: String) {
        self.direction = value;
    }

    /// Takes ownership of direction, replacing it with an empty string
    pub fn take_direction(&mut self) -> String {
        std::mem::take(&mut self.direction)
    }

    /// Returns is_exception
    pub fn is_exception(&self) -> bool {
        self.is_exception
    }

    /// Sets is_exception
    pub fn set_is_exception(&mut self, value: bool) {
        self.is_exception = value;
    }

    /// Returns is_stream
    pub fn is_stream(&self) -> bool {
        self.is_stream
    }

    /// Sets is_stream
    pub fn set_is_stream(&mut self, value: bool) {
        self.is_stream = value;
    }

    /// Returns a clone of effect if present
    pub fn effect(&self) -> Option<String> {
        self.effect.clone()
    }

    /// Sets effect
    pub fn set_effect(&mut self, value: String) {
        self.effect = Some(value);
    }

    /// Takes effect, leaving None in its place
    pub fn take_effect(&mut self) -> Option<String> {
        self.effect.take()
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
        "Parameter".to_string()
    }

}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

