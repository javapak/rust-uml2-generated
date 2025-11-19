use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::structural_feature::StructuralFeature;
use crate::value_specification::ValueSpecification;
use crate::instance_specification::InstanceSpecification;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    defining_feature: Rc<RefCell<StructuralFeature>>,
    value: Vec<ValueSpecification>,
    owning_instance: Weak<RefCell<InstanceSpecification>>,
}

impl Slot {
    pub fn new(defining_feature: Rc<RefCell<StructuralFeature>>, owning_instance: Weak<RefCell<InstanceSpecification>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            defining_feature: defining_feature,
            value: Vec::new(),
            owning_instance: owning_instance,
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

    /// Returns a reference to defining_feature
    pub fn defining_feature(&self) -> &Rc<RefCell<StructuralFeature>> {
        &self.defining_feature
    }

    /// Returns a mutable reference to defining_feature
    pub fn defining_feature_mut(&mut self) -> &mut Rc<RefCell<StructuralFeature>> {
        &mut self.defining_feature
    }

    /// Sets defining_feature
    pub fn set_defining_feature(&mut self, value: Rc<RefCell<StructuralFeature>>) {
        self.defining_feature = value;
    }

    /// Returns a slice of value
    pub fn value(&self) -> &[ValueSpecification] {
        &self.value
    }

    /// Returns a mutable reference to value
    pub fn value_mut(&mut self) -> &mut Vec<ValueSpecification> {
        &mut self.value
    }

    /// Adds an item to value
    pub fn add_value(&mut self, item: ValueSpecification) {
        self.value.push(item);
    }

    /// Clears all items from value
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    /// Returns a reference to owning_instance
    pub fn owning_instance(&self) -> &Weak<RefCell<InstanceSpecification>> {
        &self.owning_instance
    }

    /// Returns a mutable reference to owning_instance
    pub fn owning_instance_mut(&mut self) -> &mut Weak<RefCell<InstanceSpecification>> {
        &mut self.owning_instance
    }

    /// Sets owning_instance
    pub fn set_owning_instance(&mut self, value: Weak<RefCell<InstanceSpecification>>) {
        self.owning_instance = value;
    }

}

impl Default for Slot {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            defining_feature: Default::default(),
            value: Vec::new(),
            owning_instance: Default::default(),
        }
    }
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot(...)")
    }
}

