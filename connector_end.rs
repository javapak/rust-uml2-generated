use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::value_specification::ValueSpecification;
use crate::property::Property;
use crate::connectable_element::ConnectableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorEnd {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    is_ordered: bool,
    is_unique: bool,
    upper_value: Option<ValueSpecification>,
    lower_value: Option<ValueSpecification>,
    part_with_port: Option<Rc<RefCell<Property>>>,
    role: Rc<RefCell<ConnectableElement>>,
}

impl ConnectorEnd {
    pub fn new(is_ordered: bool, is_unique: bool, role: Rc<RefCell<ConnectableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            part_with_port: None,
            role: role,
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

    /// Returns is_ordered
    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    /// Sets is_ordered
    pub fn set_is_ordered(&mut self, value: bool) {
        self.is_ordered = value;
    }

    /// Returns is_unique
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }

    /// Sets is_unique
    pub fn set_is_unique(&mut self, value: bool) {
        self.is_unique = value;
    }

    /// Returns a reference to upper_value if present
    pub fn upper_value(&self) -> Option<&ValueSpecification> {
        self.upper_value.as_ref()
    }

    /// Returns a mutable reference to upper_value if present
    pub fn upper_value_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.upper_value.as_mut()
    }

    /// Sets upper_value
    pub fn set_upper_value(&mut self, value: ValueSpecification) {
        self.upper_value = Some(value);
    }

    /// Takes upper_value, leaving None in its place
    pub fn take_upper_value(&mut self) -> Option<ValueSpecification> {
        self.upper_value.take()
    }

    /// Returns a reference to lower_value if present
    pub fn lower_value(&self) -> Option<&ValueSpecification> {
        self.lower_value.as_ref()
    }

    /// Returns a mutable reference to lower_value if present
    pub fn lower_value_mut(&mut self) -> Option<&mut ValueSpecification> {
        self.lower_value.as_mut()
    }

    /// Sets lower_value
    pub fn set_lower_value(&mut self, value: ValueSpecification) {
        self.lower_value = Some(value);
    }

    /// Takes lower_value, leaving None in its place
    pub fn take_lower_value(&mut self) -> Option<ValueSpecification> {
        self.lower_value.take()
    }

    /// Returns a reference to part_with_port if present
    pub fn part_with_port(&self) -> Option<&Rc<RefCell<Property>>> {
        self.part_with_port.as_ref()
    }

    /// Returns a mutable reference to part_with_port if present
    pub fn part_with_port_mut(&mut self) -> Option<&mut Rc<RefCell<Property>>> {
        self.part_with_port.as_mut()
    }

    /// Sets part_with_port
    pub fn set_part_with_port(&mut self, value: Rc<RefCell<Property>>) {
        self.part_with_port = Some(value);
    }

    /// Takes part_with_port, leaving None in its place
    pub fn take_part_with_port(&mut self) -> Option<Rc<RefCell<Property>>> {
        self.part_with_port.take()
    }

    /// Returns a reference to role
    pub fn role(&self) -> &Rc<RefCell<ConnectableElement>> {
        &self.role
    }

    /// Returns a mutable reference to role
    pub fn role_mut(&mut self) -> &mut Rc<RefCell<ConnectableElement>> {
        &mut self.role
    }

    /// Sets role
    pub fn set_role(&mut self, value: Rc<RefCell<ConnectableElement>>) {
        self.role = value;
    }

}

impl Default for ConnectorEnd {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_ordered: false,
            is_unique: false,
            upper_value: None,
            lower_value: None,
            part_with_port: None,
            role: Default::default(),
        }
    }
}

impl std::fmt::Display for ConnectorEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectorEnd(...)")
    }
}

