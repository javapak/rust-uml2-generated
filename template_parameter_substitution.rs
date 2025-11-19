use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::template_parameter::TemplateParameter;
use crate::parameterable_element::ParameterableElement;
use crate::template_binding::TemplateBinding;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameterSubstitution {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    formal: Rc<RefCell<TemplateParameter>>,
    actual: Vec<Rc<RefCell<ParameterableElement>>>,
    owned_actual: Vec<ParameterableElement>,
    template_binding: Weak<RefCell<TemplateBinding>>,
}

impl TemplateParameterSubstitution {
    pub fn new(formal: Rc<RefCell<TemplateParameter>>, actual: Vec<Rc<RefCell<ParameterableElement>>>, template_binding: Weak<RefCell<TemplateBinding>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            formal: formal,
            actual: actual,
            owned_actual: Vec::new(),
            template_binding: template_binding,
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

    /// Returns a reference to formal
    pub fn formal(&self) -> &Rc<RefCell<TemplateParameter>> {
        &self.formal
    }

    /// Returns a mutable reference to formal
    pub fn formal_mut(&mut self) -> &mut Rc<RefCell<TemplateParameter>> {
        &mut self.formal
    }

    /// Sets formal
    pub fn set_formal(&mut self, value: Rc<RefCell<TemplateParameter>>) {
        self.formal = value;
    }

    /// Returns a reference to actual
    pub fn actual(&self) -> &Vec<Rc<RefCell<ParameterableElement>>> {
        &self.actual
    }

    /// Returns a mutable reference to actual
    pub fn actual_mut(&mut self) -> &mut Vec<Rc<RefCell<ParameterableElement>>> {
        &mut self.actual
    }

    /// Adds an item to actual
    pub fn add_actual(&mut self, item: Rc<RefCell<ParameterableElement>>) {
        self.actual.push(item);
    }

    /// Clears all items from actual
    pub fn clear_actual(&mut self) {
        self.actual.clear();
    }

    /// Returns a slice of owned_actual
    pub fn owned_actual(&self) -> &[ParameterableElement] {
        &self.owned_actual
    }

    /// Returns a mutable reference to owned_actual
    pub fn owned_actual_mut(&mut self) -> &mut Vec<ParameterableElement> {
        &mut self.owned_actual
    }

    /// Adds an item to owned_actual
    pub fn add_owned_actual(&mut self, item: ParameterableElement) {
        self.owned_actual.push(item);
    }

    /// Clears all items from owned_actual
    pub fn clear_owned_actual(&mut self) {
        self.owned_actual.clear();
    }

    /// Returns a reference to template_binding
    pub fn template_binding(&self) -> &Weak<RefCell<TemplateBinding>> {
        &self.template_binding
    }

    /// Returns a mutable reference to template_binding
    pub fn template_binding_mut(&mut self) -> &mut Weak<RefCell<TemplateBinding>> {
        &mut self.template_binding
    }

    /// Sets template_binding
    pub fn set_template_binding(&mut self, value: Weak<RefCell<TemplateBinding>>) {
        self.template_binding = value;
    }

}

impl Default for TemplateParameterSubstitution {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            formal: Default::default(),
            actual: Vec::new(),
            owned_actual: Vec::new(),
            template_binding: Default::default(),
        }
    }
}

impl std::fmt::Display for TemplateParameterSubstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateParameterSubstitution(...)")
    }
}

