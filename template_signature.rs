use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::template_parameter::TemplateParameter;
use crate::templateable_element::TemplateableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSignature {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    parameter: Vec<Rc<RefCell<TemplateParameter>>>,
    owned_parameter: Vec<TemplateParameter>,
    template: Weak<RefCell<TemplateableElement>>,
}

impl TemplateSignature {
    pub fn new(parameter: Vec<Rc<RefCell<TemplateParameter>>>, template: Weak<RefCell<TemplateableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            parameter: parameter,
            owned_parameter: Vec::new(),
            template: template,
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

    /// Returns a reference to parameter
    pub fn parameter(&self) -> &Vec<Rc<RefCell<TemplateParameter>>> {
        &self.parameter
    }

    /// Returns a mutable reference to parameter
    pub fn parameter_mut(&mut self) -> &mut Vec<Rc<RefCell<TemplateParameter>>> {
        &mut self.parameter
    }

    /// Adds an item to parameter
    pub fn add_parameter(&mut self, item: Rc<RefCell<TemplateParameter>>) {
        self.parameter.push(item);
    }

    /// Clears all items from parameter
    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    /// Returns a slice of owned_parameter
    pub fn owned_parameter(&self) -> &[TemplateParameter] {
        &self.owned_parameter
    }

    /// Returns a mutable reference to owned_parameter
    pub fn owned_parameter_mut(&mut self) -> &mut Vec<TemplateParameter> {
        &mut self.owned_parameter
    }

    /// Adds an item to owned_parameter
    pub fn add_owned_parameter(&mut self, item: TemplateParameter) {
        self.owned_parameter.push(item);
    }

    /// Clears all items from owned_parameter
    pub fn clear_owned_parameter(&mut self) {
        self.owned_parameter.clear();
    }

    /// Returns a reference to template
    pub fn template(&self) -> &Weak<RefCell<TemplateableElement>> {
        &self.template
    }

    /// Returns a mutable reference to template
    pub fn template_mut(&mut self) -> &mut Weak<RefCell<TemplateableElement>> {
        &mut self.template
    }

    /// Sets template
    pub fn set_template(&mut self, value: Weak<RefCell<TemplateableElement>>) {
        self.template = value;
    }

}

impl Default for TemplateSignature {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            parameter: Vec::new(),
            owned_parameter: Vec::new(),
            template: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateSignature(...)")
    }
}

