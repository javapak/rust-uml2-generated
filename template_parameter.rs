use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::template_signature::TemplateSignature;
use crate::parameterable_element::ParameterableElement;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    signature: Weak<RefCell<TemplateSignature>>,
    parametered_element: Rc<RefCell<ParameterableElement>>,
    owned_parametered_element: Option<ParameterableElement>,
    default: Option<Rc<RefCell<ParameterableElement>>>,
    owned_default: Option<ParameterableElement>,
}

impl TemplateParameter {
    pub fn new(signature: Weak<RefCell<TemplateSignature>>, parametered_element: Rc<RefCell<ParameterableElement>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: signature,
            parametered_element: parametered_element,
            owned_parametered_element: None,
            default: None,
            owned_default: None,
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
    pub fn signature(&self) -> &Weak<RefCell<TemplateSignature>> {
        &self.signature
    }

    /// Returns a mutable reference to signature
    pub fn signature_mut(&mut self) -> &mut Weak<RefCell<TemplateSignature>> {
        &mut self.signature
    }

    /// Sets signature
    pub fn set_signature(&mut self, value: Weak<RefCell<TemplateSignature>>) {
        self.signature = value;
    }

    /// Returns a reference to parametered_element
    pub fn parametered_element(&self) -> &Rc<RefCell<ParameterableElement>> {
        &self.parametered_element
    }

    /// Returns a mutable reference to parametered_element
    pub fn parametered_element_mut(&mut self) -> &mut Rc<RefCell<ParameterableElement>> {
        &mut self.parametered_element
    }

    /// Sets parametered_element
    pub fn set_parametered_element(&mut self, value: Rc<RefCell<ParameterableElement>>) {
        self.parametered_element = value;
    }

    /// Returns a reference to owned_parametered_element if present
    pub fn owned_parametered_element(&self) -> Option<&ParameterableElement> {
        self.owned_parametered_element.as_ref()
    }

    /// Returns a mutable reference to owned_parametered_element if present
    pub fn owned_parametered_element_mut(&mut self) -> Option<&mut ParameterableElement> {
        self.owned_parametered_element.as_mut()
    }

    /// Sets owned_parametered_element
    pub fn set_owned_parametered_element(&mut self, value: ParameterableElement) {
        self.owned_parametered_element = Some(value);
    }

    /// Takes owned_parametered_element, leaving None in its place
    pub fn take_owned_parametered_element(&mut self) -> Option<ParameterableElement> {
        self.owned_parametered_element.take()
    }

    /// Returns a reference to default if present
    pub fn default(&self) -> Option<&Rc<RefCell<ParameterableElement>>> {
        self.default.as_ref()
    }

    /// Returns a mutable reference to default if present
    pub fn default_mut(&mut self) -> Option<&mut Rc<RefCell<ParameterableElement>>> {
        self.default.as_mut()
    }

    /// Sets default
    pub fn set_default(&mut self, value: Rc<RefCell<ParameterableElement>>) {
        self.default = Some(value);
    }

    /// Takes default, leaving None in its place
    pub fn take_default(&mut self) -> Option<Rc<RefCell<ParameterableElement>>> {
        self.default.take()
    }

    /// Returns a reference to owned_default if present
    pub fn owned_default(&self) -> Option<&ParameterableElement> {
        self.owned_default.as_ref()
    }

    /// Returns a mutable reference to owned_default if present
    pub fn owned_default_mut(&mut self) -> Option<&mut ParameterableElement> {
        self.owned_default.as_mut()
    }

    /// Sets owned_default
    pub fn set_owned_default(&mut self, value: ParameterableElement) {
        self.owned_default = Some(value);
    }

    /// Takes owned_default, leaving None in its place
    pub fn take_owned_default(&mut self) -> Option<ParameterableElement> {
        self.owned_default.take()
    }

}

impl Default for TemplateParameter {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: Default::default(),
            parametered_element: Default::default(),
            owned_parametered_element: None,
            default: None,
            owned_default: None,
        }
    }
}

impl std::fmt::Display for TemplateParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateParameter(...)")
    }
}

