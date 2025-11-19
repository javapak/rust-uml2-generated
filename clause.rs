use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::executable_node::ExecutableNode;
use crate::output_pin::OutputPin;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clause {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    test: Vec<Rc<RefCell<ExecutableNode>>>,
    body: Vec<Rc<RefCell<ExecutableNode>>>,
    predecessor_clause: Vec<Weak<RefCell<Clause>>>,
    successor_clause: Vec<Weak<RefCell<Clause>>>,
    decider: Rc<RefCell<OutputPin>>,
    body_output: Vec<Rc<RefCell<OutputPin>>>,
}

impl Clause {
    pub fn new(decider: Rc<RefCell<OutputPin>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            test: Vec::new(),
            body: Vec::new(),
            predecessor_clause: Vec::new(),
            successor_clause: Vec::new(),
            decider: decider,
            body_output: Vec::new(),
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

    /// Returns a reference to test
    pub fn test(&self) -> &Vec<Rc<RefCell<ExecutableNode>>> {
        &self.test
    }

    /// Returns a mutable reference to test
    pub fn test_mut(&mut self) -> &mut Vec<Rc<RefCell<ExecutableNode>>> {
        &mut self.test
    }

    /// Adds an item to test
    pub fn add_test(&mut self, item: Rc<RefCell<ExecutableNode>>) {
        self.test.push(item);
    }

    /// Clears all items from test
    pub fn clear_test(&mut self) {
        self.test.clear();
    }

    /// Returns a reference to body
    pub fn body(&self) -> &Vec<Rc<RefCell<ExecutableNode>>> {
        &self.body
    }

    /// Returns a mutable reference to body
    pub fn body_mut(&mut self) -> &mut Vec<Rc<RefCell<ExecutableNode>>> {
        &mut self.body
    }

    /// Adds an item to body
    pub fn add_body(&mut self, item: Rc<RefCell<ExecutableNode>>) {
        self.body.push(item);
    }

    /// Clears all items from body
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    /// Returns a reference to predecessor_clause
    pub fn predecessor_clause(&self) -> &Vec<Weak<RefCell<Clause>>> {
        &self.predecessor_clause
    }

    /// Returns a mutable reference to predecessor_clause
    pub fn predecessor_clause_mut(&mut self) -> &mut Vec<Weak<RefCell<Clause>>> {
        &mut self.predecessor_clause
    }

    /// Adds an item to predecessor_clause
    pub fn add_predecessor_clause(&mut self, item: Weak<RefCell<Clause>>) {
        self.predecessor_clause.push(item);
    }

    /// Clears all items from predecessor_clause
    pub fn clear_predecessor_clause(&mut self) {
        self.predecessor_clause.clear();
    }

    /// Returns a reference to successor_clause
    pub fn successor_clause(&self) -> &Vec<Weak<RefCell<Clause>>> {
        &self.successor_clause
    }

    /// Returns a mutable reference to successor_clause
    pub fn successor_clause_mut(&mut self) -> &mut Vec<Weak<RefCell<Clause>>> {
        &mut self.successor_clause
    }

    /// Adds an item to successor_clause
    pub fn add_successor_clause(&mut self, item: Weak<RefCell<Clause>>) {
        self.successor_clause.push(item);
    }

    /// Clears all items from successor_clause
    pub fn clear_successor_clause(&mut self) {
        self.successor_clause.clear();
    }

    /// Returns a reference to decider
    pub fn decider(&self) -> &Rc<RefCell<OutputPin>> {
        &self.decider
    }

    /// Returns a mutable reference to decider
    pub fn decider_mut(&mut self) -> &mut Rc<RefCell<OutputPin>> {
        &mut self.decider
    }

    /// Sets decider
    pub fn set_decider(&mut self, value: Rc<RefCell<OutputPin>>) {
        self.decider = value;
    }

    /// Returns a reference to body_output
    pub fn body_output(&self) -> &Vec<Rc<RefCell<OutputPin>>> {
        &self.body_output
    }

    /// Returns a mutable reference to body_output
    pub fn body_output_mut(&mut self) -> &mut Vec<Rc<RefCell<OutputPin>>> {
        &mut self.body_output
    }

    /// Adds an item to body_output
    pub fn add_body_output(&mut self, item: Rc<RefCell<OutputPin>>) {
        self.body_output.push(item);
    }

    /// Clears all items from body_output
    pub fn clear_body_output(&mut self) {
        self.body_output.clear();
    }

}

impl Default for Clause {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            test: Vec::new(),
            body: Vec::new(),
            predecessor_clause: Vec::new(),
            successor_clause: Vec::new(),
            decider: Default::default(),
            body_output: Vec::new(),
        }
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clause(...)")
    }
}

