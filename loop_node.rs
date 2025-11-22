// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           LoopNode (struct)
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
use crate::activity_edge::ActivityEdge;
use crate::activity_node::ActivityNode;
use crate::exception_handler::ExceptionHandler;
use crate::constraint::Constraint;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::variable::Variable;
use crate::output_pin::OutputPin;
use crate::input_pin::InputPin;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LoopNode {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<String>,
    name_expression: Option<StringExpression>,
    is_leaf: bool,
    in_structured_node: Option<String>,
    activity: Option<String>,
    outgoing: Vec<String>,
    incoming: Vec<String>,
    in_partition: Vec<String>,
    in_interruptible_region: Vec<String>,
    redefined_node: Vec<String>,
    handler: Vec<ExceptionHandler>,
    local_precondition: Vec<Constraint>,
    local_postcondition: Vec<Constraint>,
    element_import: Vec<ElementImport>,
    package_import: Vec<PackageImport>,
    owned_rule: Vec<Constraint>,
    in_activity: Option<String>,
    variable: Vec<Variable>,
    edge: Vec<ActivityEdge>,
    must_isolate: bool,
    node: Vec<ActivityNode>,
    is_tested_first: bool,
    body_part: Vec<String>,
    setup_part: Vec<String>,
    decider: String,
    test: Vec<String>,
    result: Vec<OutputPin>,
    loop_variable: Vec<String>,
    body_output: Vec<String>,
    loop_variable_input: Vec<InputPin>,
}

#[wasm_bindgen]
impl LoopNode {
    pub fn new(is_leaf: bool, must_isolate: bool, is_tested_first: bool, decider: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            in_activity: None,
            variable: Vec::new(),
            edge: Vec::new(),
            must_isolate: must_isolate,
            node: Vec::new(),
            is_tested_first: is_tested_first,
            body_part: Vec::new(),
            setup_part: Vec::new(),
            decider: decider,
            test: Vec::new(),
            result: Vec::new(),
            loop_variable: Vec::new(),
            body_output: Vec::new(),
            loop_variable_input: Vec::new(),
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

    /// Returns a clone of in_structured_node if present
    pub fn in_structured_node(&self) -> Option<String> {
        self.in_structured_node.clone()
    }

    /// Sets in_structured_node
    pub fn set_in_structured_node(&mut self, value: String) {
        self.in_structured_node = Some(value);
    }

    /// Takes in_structured_node, leaving None in its place
    pub fn take_in_structured_node(&mut self) -> Option<String> {
        self.in_structured_node.take()
    }

    /// Returns a clone of activity if present
    pub fn activity(&self) -> Option<String> {
        self.activity.clone()
    }

    /// Sets activity
    pub fn set_activity(&mut self, value: String) {
        self.activity = Some(value);
    }

    /// Takes activity, leaving None in its place
    pub fn take_activity(&mut self) -> Option<String> {
        self.activity.take()
    }

    /// Returns a clone of outgoing
    pub fn outgoing(&self) -> Vec<String> {
        self.outgoing.clone()
    }

    /// Adds an existing ActivityEdge to outgoing by ID
    pub fn add_outgoing_by_id(&mut self, id: String) {
        self.outgoing.push(id);
    }

    /// Clears all items from outgoing
    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }

    /// Returns a clone of incoming
    pub fn incoming(&self) -> Vec<String> {
        self.incoming.clone()
    }

    /// Adds an existing ActivityEdge to incoming by ID
    pub fn add_incoming_by_id(&mut self, id: String) {
        self.incoming.push(id);
    }

    /// Clears all items from incoming
    pub fn clear_incoming(&mut self) {
        self.incoming.clear();
    }

    /// Returns a clone of in_partition
    pub fn in_partition(&self) -> Vec<String> {
        self.in_partition.clone()
    }

    /// Adds an existing ActivityPartition to in_partition by ID
    pub fn add_in_partition_by_id(&mut self, id: String) {
        self.in_partition.push(id);
    }

    /// Clears all items from in_partition
    pub fn clear_in_partition(&mut self) {
        self.in_partition.clear();
    }

    /// Returns a clone of in_interruptible_region
    pub fn in_interruptible_region(&self) -> Vec<String> {
        self.in_interruptible_region.clone()
    }

    /// Adds an existing InterruptibleActivityRegion to in_interruptible_region by ID
    pub fn add_in_interruptible_region_by_id(&mut self, id: String) {
        self.in_interruptible_region.push(id);
    }

    /// Clears all items from in_interruptible_region
    pub fn clear_in_interruptible_region(&mut self) {
        self.in_interruptible_region.clear();
    }

    /// Returns a clone of redefined_node
    pub fn redefined_node(&self) -> Vec<String> {
        self.redefined_node.clone()
    }

    /// Adds an existing ActivityNode to redefined_node by ID
    pub fn add_redefined_node_by_id(&mut self, id: String) {
        self.redefined_node.push(id);
    }

    /// Clears all items from redefined_node
    pub fn clear_redefined_node(&mut self) {
        self.redefined_node.clear();
    }

    /// Returns a clone of in_activity if present
    pub fn in_activity(&self) -> Option<String> {
        self.in_activity.clone()
    }

    /// Sets in_activity
    pub fn set_in_activity(&mut self, value: String) {
        self.in_activity = Some(value);
    }

    /// Takes in_activity, leaving None in its place
    pub fn take_in_activity(&mut self) -> Option<String> {
        self.in_activity.take()
    }

    /// Returns must_isolate
    pub fn must_isolate(&self) -> bool {
        self.must_isolate
    }

    /// Sets must_isolate
    pub fn set_must_isolate(&mut self, value: bool) {
        self.must_isolate = value;
    }

    /// Returns is_tested_first
    pub fn is_tested_first(&self) -> bool {
        self.is_tested_first
    }

    /// Sets is_tested_first
    pub fn set_is_tested_first(&mut self, value: bool) {
        self.is_tested_first = value;
    }

    /// Returns a clone of body_part
    pub fn body_part(&self) -> Vec<String> {
        self.body_part.clone()
    }

    /// Adds an existing ExecutableNode to body_part by ID
    pub fn add_body_part_by_id(&mut self, id: String) {
        self.body_part.push(id);
    }

    /// Clears all items from body_part
    pub fn clear_body_part(&mut self) {
        self.body_part.clear();
    }

    /// Returns a clone of setup_part
    pub fn setup_part(&self) -> Vec<String> {
        self.setup_part.clone()
    }

    /// Adds an existing ExecutableNode to setup_part by ID
    pub fn add_setup_part_by_id(&mut self, id: String) {
        self.setup_part.push(id);
    }

    /// Clears all items from setup_part
    pub fn clear_setup_part(&mut self) {
        self.setup_part.clear();
    }

    /// Returns a clone of decider
    pub fn decider(&self) -> String {
        self.decider.clone()
    }

    /// Sets decider
    pub fn set_decider(&mut self, value: String) {
        self.decider = value;
    }

    /// Takes ownership of decider, replacing it with an empty string
    pub fn take_decider(&mut self) -> String {
        std::mem::take(&mut self.decider)
    }

    /// Returns a clone of test
    pub fn test(&self) -> Vec<String> {
        self.test.clone()
    }

    /// Adds an existing ExecutableNode to test by ID
    pub fn add_test_by_id(&mut self, id: String) {
        self.test.push(id);
    }

    /// Clears all items from test
    pub fn clear_test(&mut self) {
        self.test.clear();
    }

    /// Returns a clone of loop_variable
    pub fn loop_variable(&self) -> Vec<String> {
        self.loop_variable.clone()
    }

    /// Adds an existing OutputPin to loop_variable by ID
    pub fn add_loop_variable_by_id(&mut self, id: String) {
        self.loop_variable.push(id);
    }

    /// Clears all items from loop_variable
    pub fn clear_loop_variable(&mut self) {
        self.loop_variable.clear();
    }

    /// Returns a clone of body_output
    pub fn body_output(&self) -> Vec<String> {
        self.body_output.clone()
    }

    /// Adds an existing OutputPin to body_output by ID
    pub fn add_body_output_by_id(&mut self, id: String) {
        self.body_output.push(id);
    }

    /// Clears all items from body_output
    pub fn clear_body_output(&mut self) {
        self.body_output.clear();
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
        "LoopNode".to_string()
    }

}

impl Default for LoopNode {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: false,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            in_activity: None,
            variable: Vec::new(),
            edge: Vec::new(),
            must_isolate: false,
            node: Vec::new(),
            is_tested_first: false,
            body_part: Vec::new(),
            setup_part: Vec::new(),
            decider: Default::default(),
            test: Vec::new(),
            result: Vec::new(),
            loop_variable: Vec::new(),
            body_output: Vec::new(),
            loop_variable_input: Vec::new(),
        }
    }
}

impl std::fmt::Display for LoopNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

