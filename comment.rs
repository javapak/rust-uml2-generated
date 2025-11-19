use crate::eannotation::EAnnotation;
use crate::element::Element;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Box<Comment>>,
    body: Option<String>,
    annotated_element: Vec<Rc<RefCell<Element>>>,
}

impl Comment {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            body: None,
            annotated_element: Vec::new(),
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

    /// Returns a reference to owned_comment
    pub fn owned_comment(&self) -> &Vec<Box<Comment>> {
        &self.owned_comment
    }

    /// Returns a mutable reference to owned_comment
    pub fn owned_comment_mut(&mut self) -> &mut Vec<Box<Comment>> {
        &mut self.owned_comment
    }

    /// Adds an item to owned_comment
    pub fn add_owned_comment(&mut self, item: Comment) {
        self.owned_comment.push(Box::new(item));
    }

    /// Clears all items from owned_comment
    pub fn clear_owned_comment(&mut self) {
        self.owned_comment.clear();
    }

    /// Returns a reference to body if present
    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    /// Returns a mutable reference to body if present
    pub fn body_mut(&mut self) -> Option<&mut String> {
        self.body.as_mut()
    }

    /// Sets body
    pub fn set_body(&mut self, value: String) {
        self.body = Some(value);
    }

    /// Takes body, leaving None in its place
    pub fn take_body(&mut self) -> Option<String> {
        self.body.take()
    }

    /// Returns a reference to annotated_element
    pub fn annotated_element(&self) -> &Vec<Rc<RefCell<Element>>> {
        &self.annotated_element
    }

    /// Returns a mutable reference to annotated_element
    pub fn annotated_element_mut(&mut self) -> &mut Vec<Rc<RefCell<Element>>> {
        &mut self.annotated_element
    }

    /// Adds an item to annotated_element
    pub fn add_annotated_element(&mut self, item: Rc<RefCell<Element>>) {
        self.annotated_element.push(item);
    }

    /// Clears all items from annotated_element
    pub fn clear_annotated_element(&mut self) {
        self.annotated_element.clear();
    }

}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comment(...)")
    }
}

