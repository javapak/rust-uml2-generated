// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Node (struct)
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
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::generalization::Generalization;
use crate::classifier::Classifier;
use crate::substitution::Substitution;
use crate::collaboration_use::CollaborationUse;
use crate::use_case::UseCase;
use crate::property::Property;
use crate::connector::Connector;
use crate::behavior::Behavior;
use crate::interface_realization::InterfaceRealization;
use crate::trigger::Trigger;
use crate::operation::Operation;
use crate::reception::Reception;
use crate::deployment::Deployment;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Node {
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
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    template_binding: Vec<TemplateBinding>,
    owned_template_signature: Option<TemplateSignature>,
    is_abstract: bool,
    generalization: Vec<Generalization>,
    powertype_extent: Vec<String>,
    redefined_classifier: Vec<String>,
    substitution: Vec<Substitution>,
    representation: Option<String>,
    collaboration_use: Vec<CollaborationUse>,
    owned_use_case: Vec<UseCase>,
    use_case: Vec<String>,
    owned_attribute: Vec<Property>,
    owned_connector: Vec<Connector>,
    owned_behavior: Vec<Behavior>,
    classifier_behavior: Option<String>,
    interface_realization: Vec<InterfaceRealization>,
    owned_trigger: Vec<Trigger>,
    nested_classifier: Vec<Classifier>,
    owned_operation: Vec<Operation>,
    is_active: bool,
    owned_reception: Vec<Reception>,
    deployment: Vec<Deployment>,
    nested_node: Vec<Box<Node>>,
}

#[wasm_bindgen]
impl Node {
    pub fn new(is_leaf: bool, is_abstract: bool, is_active: bool) -> Self {
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
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            is_abstract: is_abstract,
            generalization: Vec::new(),
            powertype_extent: Vec::new(),
            redefined_classifier: Vec::new(),
            substitution: Vec::new(),
            representation: None,
            collaboration_use: Vec::new(),
            owned_use_case: Vec::new(),
            use_case: Vec::new(),
            owned_attribute: Vec::new(),
            owned_connector: Vec::new(),
            owned_behavior: Vec::new(),
            classifier_behavior: None,
            interface_realization: Vec::new(),
            owned_trigger: Vec::new(),
            nested_classifier: Vec::new(),
            owned_operation: Vec::new(),
            is_active: is_active,
            owned_reception: Vec::new(),
            deployment: Vec::new(),
            nested_node: Vec::new(),
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

    /// Returns is_abstract
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    /// Sets is_abstract
    pub fn set_is_abstract(&mut self, value: bool) {
        self.is_abstract = value;
    }

    /// Returns a clone of powertype_extent
    pub fn powertype_extent(&self) -> Vec<String> {
        self.powertype_extent.clone()
    }

    /// Adds an existing GeneralizationSet to powertype_extent by ID
    pub fn add_powertype_extent_by_id(&mut self, id: String) {
        self.powertype_extent.push(id);
    }

    /// Clears all items from powertype_extent
    pub fn clear_powertype_extent(&mut self) {
        self.powertype_extent.clear();
    }

    /// Returns a clone of redefined_classifier
    pub fn redefined_classifier(&self) -> Vec<String> {
        self.redefined_classifier.clone()
    }

    /// Adds an existing Classifier to redefined_classifier by ID
    pub fn add_redefined_classifier_by_id(&mut self, id: String) {
        self.redefined_classifier.push(id);
    }

    /// Clears all items from redefined_classifier
    pub fn clear_redefined_classifier(&mut self) {
        self.redefined_classifier.clear();
    }

    /// Returns a clone of representation if present
    pub fn representation(&self) -> Option<String> {
        self.representation.clone()
    }

    /// Sets representation
    pub fn set_representation(&mut self, value: String) {
        self.representation = Some(value);
    }

    /// Takes representation, leaving None in its place
    pub fn take_representation(&mut self) -> Option<String> {
        self.representation.take()
    }

    /// Returns a clone of use_case
    pub fn use_case(&self) -> Vec<String> {
        self.use_case.clone()
    }

    /// Adds an existing UseCase to use_case by ID
    pub fn add_use_case_by_id(&mut self, id: String) {
        self.use_case.push(id);
    }

    /// Clears all items from use_case
    pub fn clear_use_case(&mut self) {
        self.use_case.clear();
    }

    /// Returns a clone of classifier_behavior if present
    pub fn classifier_behavior(&self) -> Option<String> {
        self.classifier_behavior.clone()
    }

    /// Sets classifier_behavior
    pub fn set_classifier_behavior(&mut self, value: String) {
        self.classifier_behavior = Some(value);
    }

    /// Takes classifier_behavior, leaving None in its place
    pub fn take_classifier_behavior(&mut self) -> Option<String> {
        self.classifier_behavior.take()
    }

    /// Returns is_active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Sets is_active
    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = value;
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
        "Node".to_string()
    }

}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

