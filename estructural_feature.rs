use crate::eannotation::EAnnotation;
use crate::eclassifier::EClassifier;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EStructuralFeature {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ordered: Option<bool>,
    unique: Option<bool>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    e_type: Option<Rc<RefCell<EClassifier>>>,
    changeable: Option<bool>,
    volatile: Option<bool>,
    transient: Option<bool>,
    default_value_literal: Option<String>,
    unsettable: Option<bool>,
    derived: Option<bool>,
}

impl EStructuralFeature {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ordered: None,
            unique: None,
            lower_bound: None,
            upper_bound: None,
            e_type: None,
            changeable: None,
            volatile: None,
            transient: None,
            default_value_literal: None,
            unsettable: None,
            derived: None,
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

    /// Returns a reference to changeable if present
    pub fn changeable(&self) -> Option<&bool> {
        self.changeable.as_ref()
    }

    /// Returns a mutable reference to changeable if present
    pub fn changeable_mut(&mut self) -> Option<&mut bool> {
        self.changeable.as_mut()
    }

    /// Sets changeable
    pub fn set_changeable(&mut self, value: bool) {
        self.changeable = Some(value);
    }

    /// Takes changeable, leaving None in its place
    pub fn take_changeable(&mut self) -> Option<bool> {
        self.changeable.take()
    }

    /// Returns a reference to volatile if present
    pub fn volatile(&self) -> Option<&bool> {
        self.volatile.as_ref()
    }

    /// Returns a mutable reference to volatile if present
    pub fn volatile_mut(&mut self) -> Option<&mut bool> {
        self.volatile.as_mut()
    }

    /// Sets volatile
    pub fn set_volatile(&mut self, value: bool) {
        self.volatile = Some(value);
    }

    /// Takes volatile, leaving None in its place
    pub fn take_volatile(&mut self) -> Option<bool> {
        self.volatile.take()
    }

    /// Returns a reference to transient if present
    pub fn transient(&self) -> Option<&bool> {
        self.transient.as_ref()
    }

    /// Returns a mutable reference to transient if present
    pub fn transient_mut(&mut self) -> Option<&mut bool> {
        self.transient.as_mut()
    }

    /// Sets transient
    pub fn set_transient(&mut self, value: bool) {
        self.transient = Some(value);
    }

    /// Takes transient, leaving None in its place
    pub fn take_transient(&mut self) -> Option<bool> {
        self.transient.take()
    }

    /// Returns a reference to default_value_literal if present
    pub fn default_value_literal(&self) -> Option<&String> {
        self.default_value_literal.as_ref()
    }

    /// Returns a mutable reference to default_value_literal if present
    pub fn default_value_literal_mut(&mut self) -> Option<&mut String> {
        self.default_value_literal.as_mut()
    }

    /// Sets default_value_literal
    pub fn set_default_value_literal(&mut self, value: String) {
        self.default_value_literal = Some(value);
    }

    /// Takes default_value_literal, leaving None in its place
    pub fn take_default_value_literal(&mut self) -> Option<String> {
        self.default_value_literal.take()
    }

    /// Returns a reference to unsettable if present
    pub fn unsettable(&self) -> Option<&bool> {
        self.unsettable.as_ref()
    }

    /// Returns a mutable reference to unsettable if present
    pub fn unsettable_mut(&mut self) -> Option<&mut bool> {
        self.unsettable.as_mut()
    }

    /// Sets unsettable
    pub fn set_unsettable(&mut self, value: bool) {
        self.unsettable = Some(value);
    }

    /// Takes unsettable, leaving None in its place
    pub fn take_unsettable(&mut self) -> Option<bool> {
        self.unsettable.take()
    }

    /// Returns a reference to derived if present
    pub fn derived(&self) -> Option<&bool> {
        self.derived.as_ref()
    }

    /// Returns a mutable reference to derived if present
    pub fn derived_mut(&mut self) -> Option<&mut bool> {
        self.derived.as_mut()
    }

    /// Sets derived
    pub fn set_derived(&mut self, value: bool) {
        self.derived = Some(value);
    }

    /// Takes derived, leaving None in its place
    pub fn take_derived(&mut self) -> Option<bool> {
        self.derived.take()
    }

}

