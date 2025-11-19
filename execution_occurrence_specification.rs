use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::lifeline::Lifeline;
use crate::general_ordering::GeneralOrdering;
use crate::interaction::Interaction;
use crate::interaction_operand::InteractionOperand;
use crate::event::Event;
use crate::execution_specification::ExecutionSpecification;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOccurrenceSpecification {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    covered: Vec<Rc<RefCell<Lifeline>>>,
    general_ordering: Vec<GeneralOrdering>,
    enclosing_interaction: Option<Weak<RefCell<Interaction>>>,
    enclosing_operand: Option<Weak<RefCell<InteractionOperand>>>,
    to_before: Vec<Rc<RefCell<GeneralOrdering>>>,
    event: Rc<RefCell<Event>>,
    to_after: Vec<Rc<RefCell<GeneralOrdering>>>,
    execution: Rc<RefCell<ExecutionSpecification>>,
}

impl ExecutionOccurrenceSpecification {
    pub fn new(event: Rc<RefCell<Event>>, execution: Rc<RefCell<ExecutionSpecification>>) -> Self {
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
            to_before: Vec::new(),
            event: event,
            to_after: Vec::new(),
            execution: execution,
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

    /// Returns a reference to covered
    pub fn covered(&self) -> &Vec<Rc<RefCell<Lifeline>>> {
        &self.covered
    }

    /// Returns a mutable reference to covered
    pub fn covered_mut(&mut self) -> &mut Vec<Rc<RefCell<Lifeline>>> {
        &mut self.covered
    }

    /// Adds an item to covered
    pub fn add_covered(&mut self, item: Rc<RefCell<Lifeline>>) {
        self.covered.push(item);
    }

    /// Clears all items from covered
    pub fn clear_covered(&mut self) {
        self.covered.clear();
    }

    /// Returns a slice of general_ordering
    pub fn general_ordering(&self) -> &[GeneralOrdering] {
        &self.general_ordering
    }

    /// Returns a mutable reference to general_ordering
    pub fn general_ordering_mut(&mut self) -> &mut Vec<GeneralOrdering> {
        &mut self.general_ordering
    }

    /// Adds an item to general_ordering
    pub fn add_general_ordering(&mut self, item: GeneralOrdering) {
        self.general_ordering.push(item);
    }

    /// Clears all items from general_ordering
    pub fn clear_general_ordering(&mut self) {
        self.general_ordering.clear();
    }

    /// Returns a reference to enclosing_interaction if present
    pub fn enclosing_interaction(&self) -> Option<&Weak<RefCell<Interaction>>> {
        self.enclosing_interaction.as_ref()
    }

    /// Returns a mutable reference to enclosing_interaction if present
    pub fn enclosing_interaction_mut(&mut self) -> Option<&mut Weak<RefCell<Interaction>>> {
        self.enclosing_interaction.as_mut()
    }

    /// Sets enclosing_interaction
    pub fn set_enclosing_interaction(&mut self, value: Weak<RefCell<Interaction>>) {
        self.enclosing_interaction = Some(value);
    }

    /// Takes enclosing_interaction, leaving None in its place
    pub fn take_enclosing_interaction(&mut self) -> Option<Weak<RefCell<Interaction>>> {
        self.enclosing_interaction.take()
    }

    /// Returns a reference to enclosing_operand if present
    pub fn enclosing_operand(&self) -> Option<&Weak<RefCell<InteractionOperand>>> {
        self.enclosing_operand.as_ref()
    }

    /// Returns a mutable reference to enclosing_operand if present
    pub fn enclosing_operand_mut(&mut self) -> Option<&mut Weak<RefCell<InteractionOperand>>> {
        self.enclosing_operand.as_mut()
    }

    /// Sets enclosing_operand
    pub fn set_enclosing_operand(&mut self, value: Weak<RefCell<InteractionOperand>>) {
        self.enclosing_operand = Some(value);
    }

    /// Takes enclosing_operand, leaving None in its place
    pub fn take_enclosing_operand(&mut self) -> Option<Weak<RefCell<InteractionOperand>>> {
        self.enclosing_operand.take()
    }

    /// Returns a reference to to_before
    pub fn to_before(&self) -> &Vec<Rc<RefCell<GeneralOrdering>>> {
        &self.to_before
    }

    /// Returns a mutable reference to to_before
    pub fn to_before_mut(&mut self) -> &mut Vec<Rc<RefCell<GeneralOrdering>>> {
        &mut self.to_before
    }

    /// Adds an item to to_before
    pub fn add_to_before(&mut self, item: Rc<RefCell<GeneralOrdering>>) {
        self.to_before.push(item);
    }

    /// Clears all items from to_before
    pub fn clear_to_before(&mut self) {
        self.to_before.clear();
    }

    /// Returns a reference to event
    pub fn event(&self) -> &Rc<RefCell<Event>> {
        &self.event
    }

    /// Returns a mutable reference to event
    pub fn event_mut(&mut self) -> &mut Rc<RefCell<Event>> {
        &mut self.event
    }

    /// Sets event
    pub fn set_event(&mut self, value: Rc<RefCell<Event>>) {
        self.event = value;
    }

    /// Returns a reference to to_after
    pub fn to_after(&self) -> &Vec<Rc<RefCell<GeneralOrdering>>> {
        &self.to_after
    }

    /// Returns a mutable reference to to_after
    pub fn to_after_mut(&mut self) -> &mut Vec<Rc<RefCell<GeneralOrdering>>> {
        &mut self.to_after
    }

    /// Adds an item to to_after
    pub fn add_to_after(&mut self, item: Rc<RefCell<GeneralOrdering>>) {
        self.to_after.push(item);
    }

    /// Clears all items from to_after
    pub fn clear_to_after(&mut self) {
        self.to_after.clear();
    }

    /// Returns a reference to execution
    pub fn execution(&self) -> &Rc<RefCell<ExecutionSpecification>> {
        &self.execution
    }

    /// Returns a mutable reference to execution
    pub fn execution_mut(&mut self) -> &mut Rc<RefCell<ExecutionSpecification>> {
        &mut self.execution
    }

    /// Sets execution
    pub fn set_execution(&mut self, value: Rc<RefCell<ExecutionSpecification>>) {
        self.execution = value;
    }

}

impl Default for ExecutionOccurrenceSpecification {
    fn default() -> Self {
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
            to_before: Vec::new(),
            event: Default::default(),
            to_after: Vec::new(),
            execution: Default::default(),
        }
    }
}

impl std::fmt::Display for ExecutionOccurrenceSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

