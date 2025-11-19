use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::region::Region;
use crate::pseudostate_kind::PseudostateKind;
use crate::state_machine::StateMachine;
use crate::state::State;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pseudostate {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    container: Option<Weak<RefCell<Region>>>,
    kind: String,
    state_machine: Option<Weak<RefCell<StateMachine>>>,
    state: Option<Weak<RefCell<State>>>,
}

impl Pseudostate {
    pub fn new(kind: String) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            container: None,
            kind: kind,
            state_machine: None,
            state: None,
        }
    }

    /// Returns a slice of e_annotations
    pub fn e_annotations(&self) -> &[EAnnotation] {
        &self.e_annotations
    }

    /// Returns a mutable reference to e_annotations
    pub fn e_annotations_mut(&mut self) -> &mut Vec<EAnnotation> {
        &mut self.e_annotations
    }

    /// Adds an item to e_annotations
    pub fn add_e_annotation(&mut self, item: EAnnotation) {
        self.e_annotations.push(item);
    }

    /// Clears all items from e_annotations
    pub fn clear_e_annotations(&mut self) {
        self.e_annotations.clear();
    }

    /// Returns a slice of owned_comment
    pub fn owned_comment(&self) -> &[Comment] {
        &self.owned_comment
    }

    /// Returns a mutable reference to owned_comment
    pub fn owned_comment_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.owned_comment
    }

    /// Adds an item to owned_comment
    pub fn add_owned_comment(&mut self, item: Comment) {
        self.owned_comment.push(item);
    }

    /// Clears all items from owned_comment
    pub fn clear_owned_comment(&mut self) {
        self.owned_comment.clear();
    }

    /// Returns a reference to name if present
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns a mutable reference to name if present
    pub fn name_mut(&mut self) -> Option<&mut String> {
        self.name.as_mut()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a reference to visibility if present
    pub fn visibility(&self) -> Option<&String> {
        self.visibility.as_ref()
    }

    /// Returns a mutable reference to visibility if present
    pub fn visibility_mut(&mut self) -> Option<&mut String> {
        self.visibility.as_mut()
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: String) {
        self.visibility = Some(value);
    }

    /// Takes visibility, leaving None in its place
    pub fn take_visibility(&mut self) -> Option<String> {
        self.visibility.take()
    }

    /// Returns a reference to client_dependency
    pub fn client_dependency(&self) -> &Vec<Rc<RefCell<Dependency>>> {
        &self.client_dependency
    }

    /// Returns a mutable reference to client_dependency
    pub fn client_dependency_mut(&mut self) -> &mut Vec<Rc<RefCell<Dependency>>> {
        &mut self.client_dependency
    }

    /// Adds an item to client_dependency
    pub fn add_client_dependency(&mut self, item: Rc<RefCell<Dependency>>) {
        self.client_dependency.push(item);
    }

    /// Clears all items from client_dependency
    pub fn clear_client_dependency(&mut self) {
        self.client_dependency.clear();
    }

    /// Returns a reference to name_expression if present
    pub fn name_expression(&self) -> Option<&StringExpression> {
        self.name_expression.as_ref()
    }

    /// Returns a mutable reference to name_expression if present
    pub fn name_expression_mut(&mut self) -> Option<&mut StringExpression> {
        self.name_expression.as_mut()
    }

    /// Sets name_expression
    pub fn set_name_expression(&mut self, value: StringExpression) {
        self.name_expression = Some(value);
    }

    /// Takes name_expression, leaving None in its place
    pub fn take_name_expression(&mut self) -> Option<StringExpression> {
        self.name_expression.take()
    }

    /// Returns a reference to container if present
    pub fn container(&self) -> Option<&Weak<RefCell<Region>>> {
        self.container.as_ref()
    }

    /// Returns a mutable reference to container if present
    pub fn container_mut(&mut self) -> Option<&mut Weak<RefCell<Region>>> {
        self.container.as_mut()
    }

    /// Sets container
    pub fn set_container(&mut self, value: Weak<RefCell<Region>>) {
        self.container = Some(value);
    }

    /// Takes container, leaving None in its place
    pub fn take_container(&mut self) -> Option<Weak<RefCell<Region>>> {
        self.container.take()
    }

    /// Returns kind as a string slice
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Sets kind
    pub fn set_kind(&mut self, value: impl Into<String>) {
        self.kind = value.into();
    }

    /// Takes ownership of kind, replacing it with an empty string
    pub fn take_kind(&mut self) -> String {
        std::mem::take(&mut self.kind)
    }

    /// Returns a reference to state_machine if present
    pub fn state_machine(&self) -> Option<&Weak<RefCell<StateMachine>>> {
        self.state_machine.as_ref()
    }

    /// Returns a mutable reference to state_machine if present
    pub fn state_machine_mut(&mut self) -> Option<&mut Weak<RefCell<StateMachine>>> {
        self.state_machine.as_mut()
    }

    /// Sets state_machine
    pub fn set_state_machine(&mut self, value: Weak<RefCell<StateMachine>>) {
        self.state_machine = Some(value);
    }

    /// Takes state_machine, leaving None in its place
    pub fn take_state_machine(&mut self) -> Option<Weak<RefCell<StateMachine>>> {
        self.state_machine.take()
    }

    /// Returns a reference to state if present
    pub fn state(&self) -> Option<&Weak<RefCell<State>>> {
        self.state.as_ref()
    }

    /// Returns a mutable reference to state if present
    pub fn state_mut(&mut self) -> Option<&mut Weak<RefCell<State>>> {
        self.state.as_mut()
    }

    /// Sets state
    pub fn set_state(&mut self, value: Weak<RefCell<State>>) {
        self.state = Some(value);
    }

    /// Takes state, leaving None in its place
    pub fn take_state(&mut self) -> Option<Weak<RefCell<State>>> {
        self.state.take()
    }

}

impl std::fmt::Display for Pseudostate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

