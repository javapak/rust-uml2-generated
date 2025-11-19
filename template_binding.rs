use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::template_signature::TemplateSignature;
use crate::template_parameter_substitution::TemplateParameterSubstitution;
use crate::templateable_element::TemplateableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateBinding {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    signature: Rc<RefCell<TemplateSignature>>,
    parameter_substitution: Vec<TemplateParameterSubstitution>,
    bound_element: Weak<RefCell<TemplateableElement>>,
}

impl TemplateBinding {
    pub fn new(signature: Rc<RefCell<TemplateSignature>>, bound_element: Weak<RefCell<TemplateableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: signature,
            parameter_substitution: Vec::new(),
            bound_element: bound_element,
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

    /// Returns a reference to signature
    pub fn signature(&self) -> &Rc<RefCell<TemplateSignature>> {
        &self.signature
    }

    /// Returns a mutable reference to signature
    pub fn signature_mut(&mut self) -> &mut Rc<RefCell<TemplateSignature>> {
        &mut self.signature
    }

    /// Sets signature
    pub fn set_signature(&mut self, value: Rc<RefCell<TemplateSignature>>) {
        self.signature = value;
    }

    /// Returns a slice of parameter_substitution
    pub fn parameter_substitution(&self) -> &[TemplateParameterSubstitution] {
        &self.parameter_substitution
    }

    /// Returns a mutable reference to parameter_substitution
    pub fn parameter_substitution_mut(&mut self) -> &mut Vec<TemplateParameterSubstitution> {
        &mut self.parameter_substitution
    }

    /// Adds an item to parameter_substitution
    pub fn add_parameter_substitution(&mut self, item: TemplateParameterSubstitution) {
        self.parameter_substitution.push(item);
    }

    /// Clears all items from parameter_substitution
    pub fn clear_parameter_substitution(&mut self) {
        self.parameter_substitution.clear();
    }

    /// Returns a reference to bound_element
    pub fn bound_element(&self) -> &Weak<RefCell<TemplateableElement>> {
        &self.bound_element
    }

    /// Returns a mutable reference to bound_element
    pub fn bound_element_mut(&mut self) -> &mut Weak<RefCell<TemplateableElement>> {
        &mut self.bound_element
    }

    /// Sets bound_element
    pub fn set_bound_element(&mut self, value: Weak<RefCell<TemplateableElement>>) {
        self.bound_element = value;
    }

}

impl Default for TemplateBinding {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: Default::default(),
            parameter_substitution: Vec::new(),
            bound_element: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateBinding(...)")
    }
}

