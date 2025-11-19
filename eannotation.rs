use crate::eobject::EObject;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EAnnotation {
    e_annotations: Vec<Box<EAnnotation>>,
    source: Option<String>,
    contents: Vec<EObject>,
    references: Vec<Rc<RefCell<EObject>>>,
}

impl EAnnotation {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            source: None,
            contents: Vec::new(),
            references: Vec::new(),
        }
    }

    /// Returns a reference to e_annotations
    pub fn e_annotations(&self) -> &Vec<Box<EAnnotation>> {
        &self.e_annotations
    }

    /// Returns a mutable reference to e_annotations
    pub fn e_annotations_mut(&mut self) -> &mut Vec<Box<EAnnotation>> {
        &mut self.e_annotations
    }

    /// Adds an item to e_annotations
    pub fn add_e_annotation(&mut self, item: EAnnotation) {
        self.e_annotations.push(Box::new(item));
    }

    /// Clears all items from e_annotations
    pub fn clear_e_annotations(&mut self) {
        self.e_annotations.clear();
    }

    /// Returns a reference to source if present
    pub fn source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    /// Returns a mutable reference to source if present
    pub fn source_mut(&mut self) -> Option<&mut String> {
        self.source.as_mut()
    }

    /// Sets source
    pub fn set_source(&mut self, value: String) {
        self.source = Some(value);
    }

    /// Takes source, leaving None in its place
    pub fn take_source(&mut self) -> Option<String> {
        self.source.take()
    }

    /// Returns a slice of contents
    pub fn contents(&self) -> &[EObject] {
        &self.contents
    }

    /// Returns a mutable reference to contents
    pub fn contents_mut(&mut self) -> &mut Vec<EObject> {
        &mut self.contents
    }

    /// Adds an item to contents
    pub fn add_content(&mut self, item: EObject) {
        self.contents.push(item);
    }

    /// Clears all items from contents
    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    /// Returns a reference to references
    pub fn references(&self) -> &Vec<Rc<RefCell<EObject>>> {
        &self.references
    }

    /// Returns a mutable reference to references
    pub fn references_mut(&mut self) -> &mut Vec<Rc<RefCell<EObject>>> {
        &mut self.references
    }

    /// Adds an item to references
    pub fn add_reference(&mut self, item: Rc<RefCell<EObject>>) {
        self.references.push(item);
    }

    /// Clears all items from references
    pub fn clear_references(&mut self) {
        self.references.clear();
    }

}

impl std::fmt::Display for EAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EAnnotation(...)")
    }
}

