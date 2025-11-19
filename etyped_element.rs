use crate::eannotation::EAnnotation;
use crate::eclassifier::EClassifier;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ETypedElement {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ordered: Option<bool>,
    unique: Option<bool>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    e_type: Option<Rc<RefCell<EClassifier>>>,
}

impl ETypedElement {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ordered: None,
            unique: None,
            lower_bound: None,
            upper_bound: None,
            e_type: None,
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

    /// Returns a reference to ordered if present
    pub fn ordered(&self) -> Option<&bool> {
        self.ordered.as_ref()
    }

    /// Returns a mutable reference to ordered if present
    pub fn ordered_mut(&mut self) -> Option<&mut bool> {
        self.ordered.as_mut()
    }

    /// Sets ordered
    pub fn set_ordered(&mut self, value: bool) {
        self.ordered = Some(value);
    }

    /// Takes ordered, leaving None in its place
    pub fn take_ordered(&mut self) -> Option<bool> {
        self.ordered.take()
    }

    /// Returns a reference to unique if present
    pub fn unique(&self) -> Option<&bool> {
        self.unique.as_ref()
    }

    /// Returns a mutable reference to unique if present
    pub fn unique_mut(&mut self) -> Option<&mut bool> {
        self.unique.as_mut()
    }

    /// Sets unique
    pub fn set_unique(&mut self, value: bool) {
        self.unique = Some(value);
    }

    /// Takes unique, leaving None in its place
    pub fn take_unique(&mut self) -> Option<bool> {
        self.unique.take()
    }

    /// Returns a reference to lower_bound if present
    pub fn lower_bound(&self) -> Option<&i32> {
        self.lower_bound.as_ref()
    }

    /// Returns a mutable reference to lower_bound if present
    pub fn lower_bound_mut(&mut self) -> Option<&mut i32> {
        self.lower_bound.as_mut()
    }

    /// Sets lower_bound
    pub fn set_lower_bound(&mut self, value: i32) {
        self.lower_bound = Some(value);
    }

    /// Takes lower_bound, leaving None in its place
    pub fn take_lower_bound(&mut self) -> Option<i32> {
        self.lower_bound.take()
    }

    /// Returns a reference to upper_bound if present
    pub fn upper_bound(&self) -> Option<&i32> {
        self.upper_bound.as_ref()
    }

    /// Returns a mutable reference to upper_bound if present
    pub fn upper_bound_mut(&mut self) -> Option<&mut i32> {
        self.upper_bound.as_mut()
    }

    /// Sets upper_bound
    pub fn set_upper_bound(&mut self, value: i32) {
        self.upper_bound = Some(value);
    }

    /// Takes upper_bound, leaving None in its place
    pub fn take_upper_bound(&mut self) -> Option<i32> {
        self.upper_bound.take()
    }

    /// Returns a reference to e_type if present
    pub fn e_type(&self) -> Option<&Rc<RefCell<EClassifier>>> {
        self.e_type.as_ref()
    }

    /// Returns a mutable reference to e_type if present
    pub fn e_type_mut(&mut self) -> Option<&mut Rc<RefCell<EClassifier>>> {
        self.e_type.as_mut()
    }

    /// Sets e_type
    pub fn set_e_type(&mut self, value: Rc<RefCell<EClassifier>>) {
        self.e_type = Some(value);
    }

    /// Takes e_type, leaving None in its place
    pub fn take_e_type(&mut self) -> Option<Rc<RefCell<EClassifier>>> {
        self.e_type.take()
    }

}

