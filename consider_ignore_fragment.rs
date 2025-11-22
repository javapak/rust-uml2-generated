// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ConsiderIgnoreFragment (struct)
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
use crate::general_ordering::GeneralOrdering;
use crate::interaction_operand::InteractionOperand;
use crate::gate::Gate;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct ConsiderIgnoreFragment {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    covered: Vec<String>,
    general_ordering: Vec<GeneralOrdering>,
    enclosing_interaction: Option<String>,
    enclosing_operand: Option<String>,
    interaction_operator: String,
    operand: Vec<InteractionOperand>,
    cfragment_gate: Vec<Gate>,
    message: Vec<String>,
}

#[wasm_bindgen]
impl ConsiderIgnoreFragment {
    pub fn new(interaction_operator: String, operand: Vec<InteractionOperand>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            covered: Vec::new(),
            general_ordering: Vec::new(),
            enclosing_interaction: None,
            enclosing_operand: None,
            interaction_operator: interaction_operator,
            operand: operand,
            cfragment_gate: Vec::new(),
            message: Vec::new(),
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

    /// Returns a clone of covered
    pub fn covered(&self) -> Vec<String> {
        self.covered.clone()
    }

    /// Adds an existing Lifeline to covered by ID
    pub fn add_covered_by_id(&mut self, id: String) {
        self.covered.push(id);
    }

    /// Clears all items from covered
    pub fn clear_covered(&mut self) {
        self.covered.clear();
    }

    /// Returns a clone of enclosing_interaction if present
    pub fn enclosing_interaction(&self) -> Option<String> {
        self.enclosing_interaction.clone()
    }

    /// Sets enclosing_interaction
    pub fn set_enclosing_interaction(&mut self, value: String) {
        self.enclosing_interaction = Some(value);
    }

    /// Takes enclosing_interaction, leaving None in its place
    pub fn take_enclosing_interaction(&mut self) -> Option<String> {
        self.enclosing_interaction.take()
    }

    /// Returns a clone of enclosing_operand if present
    pub fn enclosing_operand(&self) -> Option<String> {
        self.enclosing_operand.clone()
    }

    /// Sets enclosing_operand
    pub fn set_enclosing_operand(&mut self, value: String) {
        self.enclosing_operand = Some(value);
    }

    /// Takes enclosing_operand, leaving None in its place
    pub fn take_enclosing_operand(&mut self) -> Option<String> {
        self.enclosing_operand.take()
    }

    /// Returns a clone of interaction_operator
    pub fn interaction_operator(&self) -> String {
        self.interaction_operator.clone()
    }

    /// Sets interaction_operator
    pub fn set_interaction_operator(&mut self, value: String) {
        self.interaction_operator = value;
    }

    /// Takes ownership of interaction_operator, replacing it with an empty string
    pub fn take_interaction_operator(&mut self) -> String {
        std::mem::take(&mut self.interaction_operator)
    }

    /// Returns a clone of message
    pub fn message(&self) -> Vec<String> {
        self.message.clone()
    }

    /// Adds an existing NamedElement to message by ID
    pub fn add_message_by_id(&mut self, id: String) {
        self.message.push(id);
    }

    /// Clears all items from message
    pub fn clear_message(&mut self) {
        self.message.clear();
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
        "ConsiderIgnoreFragment".to_string()
    }

}

impl std::fmt::Display for ConsiderIgnoreFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

