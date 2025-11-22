// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Operation (struct)
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
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Operation {
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
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    interface: Option<String>,
    class: Option<String>,
    is_query: bool,
    precondition: Vec<String>,
    postcondition: Vec<String>,
    redefined_operation: Vec<String>,
    datatype: Option<String>,
    body_condition: Option<String>,
}

#[wasm_bindgen]
impl Operation {
    pub fn new(is_leaf: bool, is_static: bool, is_abstract: bool, concurrency: String, is_query: bool) -> Self {
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
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            interface: None,
            class: None,
            is_query: is_query,
            precondition: Vec::new(),
            postcondition: Vec::new(),
            redefined_operation: Vec::new(),
            datatype: None,
            body_condition: None,
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

    /// Returns a clone of interface if present
    pub fn interface(&self) -> Option<String> {
        self.interface.clone()
    }

    /// Sets interface
    pub fn set_interface(&mut self, value: String) {
        self.interface = Some(value);
    }

    /// Takes interface, leaving None in its place
    pub fn take_interface(&mut self) -> Option<String> {
        self.interface.take()
    }

    /// Returns a clone of class if present
    pub fn class(&self) -> Option<String> {
        self.class.clone()
    }

    /// Sets class
    pub fn set_class(&mut self, value: String) {
        self.class = Some(value);
    }

    /// Takes class, leaving None in its place
    pub fn take_class(&mut self) -> Option<String> {
        self.class.take()
    }

    /// Returns is_query
    pub fn is_query(&self) -> bool {
        self.is_query
    }

    /// Sets is_query
    pub fn set_is_query(&mut self, value: bool) {
        self.is_query = value;
    }

    /// Returns a clone of precondition
    pub fn precondition(&self) -> Vec<String> {
        self.precondition.clone()
    }

    /// Adds an existing Constraint to precondition by ID
    pub fn add_precondition_by_id(&mut self, id: String) {
        self.precondition.push(id);
    }

    /// Clears all items from precondition
    pub fn clear_precondition(&mut self) {
        self.precondition.clear();
    }

    /// Returns a clone of postcondition
    pub fn postcondition(&self) -> Vec<String> {
        self.postcondition.clone()
    }

    /// Adds an existing Constraint to postcondition by ID
    pub fn add_postcondition_by_id(&mut self, id: String) {
        self.postcondition.push(id);
    }

    /// Clears all items from postcondition
    pub fn clear_postcondition(&mut self) {
        self.postcondition.clear();
    }

    /// Returns a clone of redefined_operation
    pub fn redefined_operation(&self) -> Vec<String> {
        self.redefined_operation.clone()
    }

    /// Adds an existing Operation to redefined_operation by ID
    pub fn add_redefined_operation_by_id(&mut self, id: String) {
        self.redefined_operation.push(id);
    }

    /// Clears all items from redefined_operation
    pub fn clear_redefined_operation(&mut self) {
        self.redefined_operation.clear();
    }

    /// Returns a clone of datatype if present
    pub fn datatype(&self) -> Option<String> {
        self.datatype.clone()
    }

    /// Sets datatype
    pub fn set_datatype(&mut self, value: String) {
        self.datatype = Some(value);
    }

    /// Takes datatype, leaving None in its place
    pub fn take_datatype(&mut self) -> Option<String> {
        self.datatype.take()
    }

    /// Returns a clone of body_condition if present
    pub fn body_condition(&self) -> Option<String> {
        self.body_condition.clone()
    }

    /// Sets body_condition
    pub fn set_body_condition(&mut self, value: String) {
        self.body_condition = Some(value);
    }

    /// Takes body_condition, leaving None in its place
    pub fn take_body_condition(&mut self) -> Option<String> {
        self.body_condition.take()
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
        "Operation".to_string()
    }

}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

