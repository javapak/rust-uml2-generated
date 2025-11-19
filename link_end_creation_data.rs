use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::input_pin::InputPin;
use crate::property::Property;
use crate::qualifier_value::QualifierValue;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkEndCreationData {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    value: Option<Rc<RefCell<InputPin>>>,
    end: Rc<RefCell<Property>>,
    qualifier: Vec<QualifierValue>,
    is_replace_all: bool,
    insert_at: Option<Rc<RefCell<InputPin>>>,
}

impl LinkEndCreationData {
    pub fn new(end: Rc<RefCell<Property>>, is_replace_all: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            value: None,
            end: end,
            qualifier: Vec::new(),
            is_replace_all: is_replace_all,
            insert_at: None,
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

    /// Returns a reference to value if present
    pub fn value(&self) -> Option<&Rc<RefCell<InputPin>>> {
        self.value.as_ref()
    }

    /// Returns a mutable reference to value if present
    pub fn value_mut(&mut self) -> Option<&mut Rc<RefCell<InputPin>>> {
        self.value.as_mut()
    }

    /// Sets value
    pub fn set_value(&mut self, value: Rc<RefCell<InputPin>>) {
        self.value = Some(value);
    }

    /// Takes value, leaving None in its place
    pub fn take_value(&mut self) -> Option<Rc<RefCell<InputPin>>> {
        self.value.take()
    }

    /// Returns a reference to end
    pub fn end(&self) -> &Rc<RefCell<Property>> {
        &self.end
    }

    /// Returns a mutable reference to end
    pub fn end_mut(&mut self) -> &mut Rc<RefCell<Property>> {
        &mut self.end
    }

    /// Sets end
    pub fn set_end(&mut self, value: Rc<RefCell<Property>>) {
        self.end = value;
    }

    /// Returns a slice of qualifier
    pub fn qualifier(&self) -> &[QualifierValue] {
        &self.qualifier
    }

    /// Returns a mutable reference to qualifier
    pub fn qualifier_mut(&mut self) -> &mut Vec<QualifierValue> {
        &mut self.qualifier
    }

    /// Adds an item to qualifier
    pub fn add_qualifier(&mut self, item: QualifierValue) {
        self.qualifier.push(item);
    }

    /// Clears all items from qualifier
    pub fn clear_qualifier(&mut self) {
        self.qualifier.clear();
    }

    /// Returns is_replace_all
    pub fn is_replace_all(&self) -> bool {
        self.is_replace_all
    }

    /// Sets is_replace_all
    pub fn set_is_replace_all(&mut self, value: bool) {
        self.is_replace_all = value;
    }

    /// Returns a reference to insert_at if present
    pub fn insert_at(&self) -> Option<&Rc<RefCell<InputPin>>> {
        self.insert_at.as_ref()
    }

    /// Returns a mutable reference to insert_at if present
    pub fn insert_at_mut(&mut self) -> Option<&mut Rc<RefCell<InputPin>>> {
        self.insert_at.as_mut()
    }

    /// Sets insert_at
    pub fn set_insert_at(&mut self, value: Rc<RefCell<InputPin>>) {
        self.insert_at = Some(value);
    }

    /// Takes insert_at, leaving None in its place
    pub fn take_insert_at(&mut self) -> Option<Rc<RefCell<InputPin>>> {
        self.insert_at.take()
    }

}

impl Default for LinkEndCreationData {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            value: None,
            end: Default::default(),
            qualifier: Vec::new(),
            is_replace_all: false,
            insert_at: None,
        }
    }
}

impl std::fmt::Display for LinkEndCreationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkEndCreationData(...)")
    }
}

