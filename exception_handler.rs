use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::executable_node::ExecutableNode;
use crate::object_node::ObjectNode;
use crate::classifier::Classifier;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionHandler {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    handler_body: Rc<RefCell<ExecutableNode>>,
    exception_input: Rc<RefCell<ObjectNode>>,
    exception_type: Vec<Rc<RefCell<Classifier>>>,
    protected_node: Weak<RefCell<ExecutableNode>>,
}

impl ExceptionHandler {
    pub fn new(handler_body: Rc<RefCell<ExecutableNode>>, exception_input: Rc<RefCell<ObjectNode>>, exception_type: Vec<Rc<RefCell<Classifier>>>, protected_node: Weak<RefCell<ExecutableNode>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            handler_body: handler_body,
            exception_input: exception_input,
            exception_type: exception_type,
            protected_node: protected_node,
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

    /// Returns a reference to handler_body
    pub fn handler_body(&self) -> &Rc<RefCell<ExecutableNode>> {
        &self.handler_body
    }

    /// Returns a mutable reference to handler_body
    pub fn handler_body_mut(&mut self) -> &mut Rc<RefCell<ExecutableNode>> {
        &mut self.handler_body
    }

    /// Sets handler_body
    pub fn set_handler_body(&mut self, value: Rc<RefCell<ExecutableNode>>) {
        self.handler_body = value;
    }

    /// Returns a reference to exception_input
    pub fn exception_input(&self) -> &Rc<RefCell<ObjectNode>> {
        &self.exception_input
    }

    /// Returns a mutable reference to exception_input
    pub fn exception_input_mut(&mut self) -> &mut Rc<RefCell<ObjectNode>> {
        &mut self.exception_input
    }

    /// Sets exception_input
    pub fn set_exception_input(&mut self, value: Rc<RefCell<ObjectNode>>) {
        self.exception_input = value;
    }

    /// Returns a reference to exception_type
    pub fn exception_type(&self) -> &Vec<Rc<RefCell<Classifier>>> {
        &self.exception_type
    }

    /// Returns a mutable reference to exception_type
    pub fn exception_type_mut(&mut self) -> &mut Vec<Rc<RefCell<Classifier>>> {
        &mut self.exception_type
    }

    /// Adds an item to exception_type
    pub fn add_exception_type(&mut self, item: Rc<RefCell<Classifier>>) {
        self.exception_type.push(item);
    }

    /// Clears all items from exception_type
    pub fn clear_exception_type(&mut self) {
        self.exception_type.clear();
    }

    /// Returns a reference to protected_node
    pub fn protected_node(&self) -> &Weak<RefCell<ExecutableNode>> {
        &self.protected_node
    }

    /// Returns a mutable reference to protected_node
    pub fn protected_node_mut(&mut self) -> &mut Weak<RefCell<ExecutableNode>> {
        &mut self.protected_node
    }

    /// Sets protected_node
    pub fn set_protected_node(&mut self, value: Weak<RefCell<ExecutableNode>>) {
        self.protected_node = value;
    }

}

impl Default for ExceptionHandler {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            handler_body: Default::default(),
            exception_input: Default::default(),
            exception_type: Vec::new(),
            protected_node: Default::default(),
        }
    }
}

impl std::fmt::Display for ExceptionHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExceptionHandler(...)")
    }
}

