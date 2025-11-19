use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::protocol_state_machine::ProtocolStateMachine;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConformance {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    general_machine: Rc<RefCell<ProtocolStateMachine>>,
    specific_machine: Weak<RefCell<ProtocolStateMachine>>,
}

impl ProtocolConformance {
    pub fn new(general_machine: Rc<RefCell<ProtocolStateMachine>>, specific_machine: Weak<RefCell<ProtocolStateMachine>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            general_machine: general_machine,
            specific_machine: specific_machine,
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

    /// Returns a reference to general_machine
    pub fn general_machine(&self) -> &Rc<RefCell<ProtocolStateMachine>> {
        &self.general_machine
    }

    /// Returns a mutable reference to general_machine
    pub fn general_machine_mut(&mut self) -> &mut Rc<RefCell<ProtocolStateMachine>> {
        &mut self.general_machine
    }

    /// Sets general_machine
    pub fn set_general_machine(&mut self, value: Rc<RefCell<ProtocolStateMachine>>) {
        self.general_machine = value;
    }

    /// Returns a reference to specific_machine
    pub fn specific_machine(&self) -> &Weak<RefCell<ProtocolStateMachine>> {
        &self.specific_machine
    }

    /// Returns a mutable reference to specific_machine
    pub fn specific_machine_mut(&mut self) -> &mut Weak<RefCell<ProtocolStateMachine>> {
        &mut self.specific_machine
    }

    /// Sets specific_machine
    pub fn set_specific_machine(&mut self, value: Weak<RefCell<ProtocolStateMachine>>) {
        self.specific_machine = value;
    }

}

impl Default for ProtocolConformance {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            general_machine: Default::default(),
            specific_machine: Default::default(),
        }
    }
}

impl std::fmt::Display for ProtocolConformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtocolConformance(...)")
    }
}

