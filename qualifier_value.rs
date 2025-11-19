use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::property::Property;
use crate::input_pin::InputPin;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualifierValue {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    qualifier: Rc<RefCell<Property>>,
    value: Rc<RefCell<InputPin>>,
}

impl QualifierValue {
    pub fn new(qualifier: Rc<RefCell<Property>>, value: Rc<RefCell<InputPin>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            qualifier: qualifier,
            value: value,
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

    /// Returns a reference to qualifier
    pub fn qualifier(&self) -> &Rc<RefCell<Property>> {
        &self.qualifier
    }

    /// Returns a mutable reference to qualifier
    pub fn qualifier_mut(&mut self) -> &mut Rc<RefCell<Property>> {
        &mut self.qualifier
    }

    /// Sets qualifier
    pub fn set_qualifier(&mut self, value: Rc<RefCell<Property>>) {
        self.qualifier = value;
    }

    /// Returns a reference to value
    pub fn value(&self) -> &Rc<RefCell<InputPin>> {
        &self.value
    }

    /// Returns a mutable reference to value
    pub fn value_mut(&mut self) -> &mut Rc<RefCell<InputPin>> {
        &mut self.value
    }

    /// Sets value
    pub fn set_value(&mut self, value: Rc<RefCell<InputPin>>) {
        self.value = value;
    }

}

impl Default for QualifierValue {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            qualifier: Default::default(),
            value: Default::default(),
        }
    }
}

impl std::fmt::Display for QualifierValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifierValue(...)")
    }
}

