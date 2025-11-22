// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Message (struct)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Message {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    message_sort: String,
    receive_event: Option<String>,
    send_event: Option<String>,
    connector: Option<String>,
    interaction: String,
    argument: Vec<ValueSpecification>,
}

#[wasm_bindgen]
impl Message {
    pub fn new(message_sort: String, interaction: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            message_sort: message_sort,
            receive_event: None,
            send_event: None,
            connector: None,
            interaction: interaction,
            argument: Vec::new(),
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

    /// Returns a clone of message_sort
    pub fn message_sort(&self) -> String {
        self.message_sort.clone()
    }

    /// Sets message_sort
    pub fn set_message_sort(&mut self, value: String) {
        self.message_sort = value;
    }

    /// Takes ownership of message_sort, replacing it with an empty string
    pub fn take_message_sort(&mut self) -> String {
        std::mem::take(&mut self.message_sort)
    }

    /// Returns a clone of receive_event if present
    pub fn receive_event(&self) -> Option<String> {
        self.receive_event.clone()
    }

    /// Sets receive_event
    pub fn set_receive_event(&mut self, value: String) {
        self.receive_event = Some(value);
    }

    /// Takes receive_event, leaving None in its place
    pub fn take_receive_event(&mut self) -> Option<String> {
        self.receive_event.take()
    }

    /// Returns a clone of send_event if present
    pub fn send_event(&self) -> Option<String> {
        self.send_event.clone()
    }

    /// Sets send_event
    pub fn set_send_event(&mut self, value: String) {
        self.send_event = Some(value);
    }

    /// Takes send_event, leaving None in its place
    pub fn take_send_event(&mut self) -> Option<String> {
        self.send_event.take()
    }

    /// Returns a clone of connector if present
    pub fn connector(&self) -> Option<String> {
        self.connector.clone()
    }

    /// Sets connector
    pub fn set_connector(&mut self, value: String) {
        self.connector = Some(value);
    }

    /// Takes connector, leaving None in its place
    pub fn take_connector(&mut self) -> Option<String> {
        self.connector.take()
    }

    /// Returns a clone of interaction
    pub fn interaction(&self) -> String {
        self.interaction.clone()
    }

    /// Sets interaction
    pub fn set_interaction(&mut self, value: String) {
        self.interaction = value;
    }

    /// Takes ownership of interaction, replacing it with an empty string
    pub fn take_interaction(&mut self) -> String {
        std::mem::take(&mut self.interaction)
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
        "Message".to_string()
    }

}

impl Default for Message {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            message_sort: String::new(),
            receive_event: None,
            send_event: None,
            connector: None,
            interaction: Default::default(),
            argument: Vec::new(),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

