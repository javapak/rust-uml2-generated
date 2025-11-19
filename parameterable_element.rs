use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::template_parameter::TemplateParameter;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParameterableElement {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    owning_template_parameter: Option<Weak<RefCell<TemplateParameter>>>,
    template_parameter: Option<Rc<RefCell<TemplateParameter>>>,
}

impl ParameterableElement {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            owning_template_parameter: None,
            template_parameter: None,
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

    /// Returns a reference to owning_template_parameter if present
    pub fn owning_template_parameter(&self) -> Option<&Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.as_ref()
    }

    /// Returns a mutable reference to owning_template_parameter if present
    pub fn owning_template_parameter_mut(&mut self) -> Option<&mut Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.as_mut()
    }

    /// Sets owning_template_parameter
    pub fn set_owning_template_parameter(&mut self, value: Weak<RefCell<TemplateParameter>>) {
        self.owning_template_parameter = Some(value);
    }

    /// Takes owning_template_parameter, leaving None in its place
    pub fn take_owning_template_parameter(&mut self) -> Option<Weak<RefCell<TemplateParameter>>> {
        self.owning_template_parameter.take()
    }

    /// Returns a reference to template_parameter if present
    pub fn template_parameter(&self) -> Option<&Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.as_ref()
    }

    /// Returns a mutable reference to template_parameter if present
    pub fn template_parameter_mut(&mut self) -> Option<&mut Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.as_mut()
    }

    /// Sets template_parameter
    pub fn set_template_parameter(&mut self, value: Rc<RefCell<TemplateParameter>>) {
        self.template_parameter = Some(value);
    }

    /// Takes template_parameter, leaving None in its place
    pub fn take_template_parameter(&mut self) -> Option<Rc<RefCell<TemplateParameter>>> {
        self.template_parameter.take()
    }

}

