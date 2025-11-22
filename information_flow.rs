// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           InformationFlow (struct)
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
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct InformationFlow {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    owning_template_parameter: Option<String>,
    template_parameter: Option<String>,
    realization: Vec<String>,
    conveyed: Vec<String>,
    information_source: Vec<String>,
    information_target: Vec<String>,
    realizing_activity_edge: Vec<String>,
    realizing_connector: Vec<String>,
    realizing_message: Vec<String>,
}

#[wasm_bindgen]
impl InformationFlow {
    pub fn new(conveyed: Vec<String>, information_source: Vec<String>, information_target: Vec<String>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            owning_template_parameter: None,
            template_parameter: None,
            realization: Vec::new(),
            conveyed: conveyed,
            information_source: information_source,
            information_target: information_target,
            realizing_activity_edge: Vec::new(),
            realizing_connector: Vec::new(),
            realizing_message: Vec::new(),
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

    /// Returns a clone of realization
    pub fn realization(&self) -> Vec<String> {
        self.realization.clone()
    }

    /// Adds an existing Relationship to realization by ID
    pub fn add_realization_by_id(&mut self, id: String) {
        self.realization.push(id);
    }

    /// Clears all items from realization
    pub fn clear_realization(&mut self) {
        self.realization.clear();
    }

    /// Returns a clone of conveyed
    pub fn conveyed(&self) -> Vec<String> {
        self.conveyed.clone()
    }

    /// Adds an existing Classifier to conveyed by ID
    pub fn add_conveyed_by_id(&mut self, id: String) {
        self.conveyed.push(id);
    }

    /// Clears all items from conveyed
    pub fn clear_conveyed(&mut self) {
        self.conveyed.clear();
    }

    /// Returns a clone of information_source
    pub fn information_source(&self) -> Vec<String> {
        self.information_source.clone()
    }

    /// Adds an existing NamedElement to information_source by ID
    pub fn add_information_source_by_id(&mut self, id: String) {
        self.information_source.push(id);
    }

    /// Clears all items from information_source
    pub fn clear_information_source(&mut self) {
        self.information_source.clear();
    }

    /// Returns a clone of information_target
    pub fn information_target(&self) -> Vec<String> {
        self.information_target.clone()
    }

    /// Adds an existing NamedElement to information_target by ID
    pub fn add_information_target_by_id(&mut self, id: String) {
        self.information_target.push(id);
    }

    /// Clears all items from information_target
    pub fn clear_information_target(&mut self) {
        self.information_target.clear();
    }

    /// Returns a clone of realizing_activity_edge
    pub fn realizing_activity_edge(&self) -> Vec<String> {
        self.realizing_activity_edge.clone()
    }

    /// Adds an existing ActivityEdge to realizing_activity_edge by ID
    pub fn add_realizing_activity_edge_by_id(&mut self, id: String) {
        self.realizing_activity_edge.push(id);
    }

    /// Clears all items from realizing_activity_edge
    pub fn clear_realizing_activity_edge(&mut self) {
        self.realizing_activity_edge.clear();
    }

    /// Returns a clone of realizing_connector
    pub fn realizing_connector(&self) -> Vec<String> {
        self.realizing_connector.clone()
    }

    /// Adds an existing Connector to realizing_connector by ID
    pub fn add_realizing_connector_by_id(&mut self, id: String) {
        self.realizing_connector.push(id);
    }

    /// Clears all items from realizing_connector
    pub fn clear_realizing_connector(&mut self) {
        self.realizing_connector.clear();
    }

    /// Returns a clone of realizing_message
    pub fn realizing_message(&self) -> Vec<String> {
        self.realizing_message.clone()
    }

    /// Adds an existing Message to realizing_message by ID
    pub fn add_realizing_message_by_id(&mut self, id: String) {
        self.realizing_message.push(id);
    }

    /// Clears all items from realizing_message
    pub fn clear_realizing_message(&mut self) {
        self.realizing_message.clear();
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
        "InformationFlow".to_string()
    }

}

impl std::fmt::Display for InformationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

