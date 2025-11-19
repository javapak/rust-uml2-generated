use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::message_sort::MessageSort;
use crate::message_end::MessageEnd;
use crate::connector::Connector;
use crate::interaction::Interaction;
use crate::value_specification::ValueSpecification;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    name: Option<String>,
    visibility: Option<String>,
    client_dependency: Vec<Rc<RefCell<Dependency>>>,
    name_expression: Option<StringExpression>,
    message_sort: String,
    receive_event: Option<Rc<RefCell<MessageEnd>>>,
    send_event: Option<Rc<RefCell<MessageEnd>>>,
    connector: Option<Rc<RefCell<Connector>>>,
    interaction: Weak<RefCell<Interaction>>,
    argument: Vec<ValueSpecification>,
}

impl Message {
    pub fn new(message_sort: String, interaction: Weak<RefCell<Interaction>>) -> Self {
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

    /// Returns message_sort as a string slice
    pub fn message_sort(&self) -> &str {
        &self.message_sort
    }

    /// Sets message_sort
    pub fn set_message_sort(&mut self, value: impl Into<String>) {
        self.message_sort = value.into();
    }

    /// Takes ownership of message_sort, replacing it with an empty string
    pub fn take_message_sort(&mut self) -> String {
        std::mem::take(&mut self.message_sort)
    }

    /// Returns a reference to receive_event if present
    pub fn receive_event(&self) -> Option<&Rc<RefCell<MessageEnd>>> {
        self.receive_event.as_ref()
    }

    /// Returns a mutable reference to receive_event if present
    pub fn receive_event_mut(&mut self) -> Option<&mut Rc<RefCell<MessageEnd>>> {
        self.receive_event.as_mut()
    }

    /// Sets receive_event
    pub fn set_receive_event(&mut self, value: Rc<RefCell<MessageEnd>>) {
        self.receive_event = Some(value);
    }

    /// Takes receive_event, leaving None in its place
    pub fn take_receive_event(&mut self) -> Option<Rc<RefCell<MessageEnd>>> {
        self.receive_event.take()
    }

    /// Returns a reference to send_event if present
    pub fn send_event(&self) -> Option<&Rc<RefCell<MessageEnd>>> {
        self.send_event.as_ref()
    }

    /// Returns a mutable reference to send_event if present
    pub fn send_event_mut(&mut self) -> Option<&mut Rc<RefCell<MessageEnd>>> {
        self.send_event.as_mut()
    }

    /// Sets send_event
    pub fn set_send_event(&mut self, value: Rc<RefCell<MessageEnd>>) {
        self.send_event = Some(value);
    }

    /// Takes send_event, leaving None in its place
    pub fn take_send_event(&mut self) -> Option<Rc<RefCell<MessageEnd>>> {
        self.send_event.take()
    }

    /// Returns a reference to connector if present
    pub fn connector(&self) -> Option<&Rc<RefCell<Connector>>> {
        self.connector.as_ref()
    }

    /// Returns a mutable reference to connector if present
    pub fn connector_mut(&mut self) -> Option<&mut Rc<RefCell<Connector>>> {
        self.connector.as_mut()
    }

    /// Sets connector
    pub fn set_connector(&mut self, value: Rc<RefCell<Connector>>) {
        self.connector = Some(value);
    }

    /// Takes connector, leaving None in its place
    pub fn take_connector(&mut self) -> Option<Rc<RefCell<Connector>>> {
        self.connector.take()
    }

    /// Returns a reference to interaction
    pub fn interaction(&self) -> &Weak<RefCell<Interaction>> {
        &self.interaction
    }

    /// Returns a mutable reference to interaction
    pub fn interaction_mut(&mut self) -> &mut Weak<RefCell<Interaction>> {
        &mut self.interaction
    }

    /// Sets interaction
    pub fn set_interaction(&mut self, value: Weak<RefCell<Interaction>>) {
        self.interaction = value;
    }

    /// Returns a slice of argument
    pub fn argument(&self) -> &[ValueSpecification] {
        &self.argument
    }

    /// Returns a mutable reference to argument
    pub fn argument_mut(&mut self) -> &mut Vec<ValueSpecification> {
        &mut self.argument
    }

    /// Adds an item to argument
    pub fn add_argument(&mut self, item: ValueSpecification) {
        self.argument.push(item);
    }

    /// Clears all items from argument
    pub fn clear_argument(&mut self) {
        self.argument.clear();
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

