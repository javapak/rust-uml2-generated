use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::classifier::Classifier;
use crate::generalization_set::GeneralizationSet;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generalization {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    is_substitutable: Option<bool>,
    general: Rc<RefCell<Classifier>>,
    generalization_set: Vec<Rc<RefCell<GeneralizationSet>>>,
    specific: Weak<RefCell<Classifier>>,
}

impl Generalization {
    pub fn new(general: Rc<RefCell<Classifier>>, specific: Weak<RefCell<Classifier>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_substitutable: None,
            general: general,
            generalization_set: Vec::new(),
            specific: specific,
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

    /// Returns a reference to is_substitutable if present
    pub fn is_substitutable(&self) -> Option<&bool> {
        self.is_substitutable.as_ref()
    }

    /// Returns a mutable reference to is_substitutable if present
    pub fn is_substitutable_mut(&mut self) -> Option<&mut bool> {
        self.is_substitutable.as_mut()
    }

    /// Sets is_substitutable
    pub fn set_is_substitutable(&mut self, value: bool) {
        self.is_substitutable = Some(value);
    }

    /// Takes is_substitutable, leaving None in its place
    pub fn take_is_substitutable(&mut self) -> Option<bool> {
        self.is_substitutable.take()
    }

    /// Returns a reference to general
    pub fn general(&self) -> &Rc<RefCell<Classifier>> {
        &self.general
    }

    /// Returns a mutable reference to general
    pub fn general_mut(&mut self) -> &mut Rc<RefCell<Classifier>> {
        &mut self.general
    }

    /// Sets general
    pub fn set_general(&mut self, value: Rc<RefCell<Classifier>>) {
        self.general = value;
    }

    /// Returns a reference to generalization_set
    pub fn generalization_set(&self) -> &Vec<Rc<RefCell<GeneralizationSet>>> {
        &self.generalization_set
    }

    /// Returns a mutable reference to generalization_set
    pub fn generalization_set_mut(&mut self) -> &mut Vec<Rc<RefCell<GeneralizationSet>>> {
        &mut self.generalization_set
    }

    /// Adds an item to generalization_set
    pub fn add_generalization_set(&mut self, item: Rc<RefCell<GeneralizationSet>>) {
        self.generalization_set.push(item);
    }

    /// Clears all items from generalization_set
    pub fn clear_generalization_set(&mut self) {
        self.generalization_set.clear();
    }

    /// Returns a reference to specific
    pub fn specific(&self) -> &Weak<RefCell<Classifier>> {
        &self.specific
    }

    /// Returns a mutable reference to specific
    pub fn specific_mut(&mut self) -> &mut Weak<RefCell<Classifier>> {
        &mut self.specific
    }

    /// Sets specific
    pub fn set_specific(&mut self, value: Weak<RefCell<Classifier>>) {
        self.specific = value;
    }

}

impl Default for Generalization {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_substitutable: None,
            general: Default::default(),
            generalization_set: Vec::new(),
            specific: Default::default(),
        }
    }
}

impl std::fmt::Display for Generalization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generalization(...)")
    }
}

