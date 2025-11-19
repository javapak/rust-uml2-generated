use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::visibility_kind::VisibilityKind;
use crate::package::Package;
use crate::namespace::Namespace;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageImport {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    visibility: String,
    imported_package: Rc<RefCell<Package>>,
    importing_namespace: Weak<RefCell<Namespace>>,
}

impl PackageImport {
    pub fn new(visibility: String, imported_package: Rc<RefCell<Package>>, importing_namespace: Weak<RefCell<Namespace>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            visibility: visibility,
            imported_package: imported_package,
            importing_namespace: importing_namespace,
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

    /// Returns visibility as a string slice
    pub fn visibility(&self) -> &str {
        &self.visibility
    }

    /// Sets visibility
    pub fn set_visibility(&mut self, value: impl Into<String>) {
        self.visibility = value.into();
    }

    /// Takes ownership of visibility, replacing it with an empty string
    pub fn take_visibility(&mut self) -> String {
        std::mem::take(&mut self.visibility)
    }

    /// Returns a reference to imported_package
    pub fn imported_package(&self) -> &Rc<RefCell<Package>> {
        &self.imported_package
    }

    /// Returns a mutable reference to imported_package
    pub fn imported_package_mut(&mut self) -> &mut Rc<RefCell<Package>> {
        &mut self.imported_package
    }

    /// Sets imported_package
    pub fn set_imported_package(&mut self, value: Rc<RefCell<Package>>) {
        self.imported_package = value;
    }

    /// Returns a reference to importing_namespace
    pub fn importing_namespace(&self) -> &Weak<RefCell<Namespace>> {
        &self.importing_namespace
    }

    /// Returns a mutable reference to importing_namespace
    pub fn importing_namespace_mut(&mut self) -> &mut Weak<RefCell<Namespace>> {
        &mut self.importing_namespace
    }

    /// Sets importing_namespace
    pub fn set_importing_namespace(&mut self, value: Weak<RefCell<Namespace>>) {
        self.importing_namespace = value;
    }

}

impl Default for PackageImport {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            visibility: String::new(),
            imported_package: Default::default(),
            importing_namespace: Default::default(),
        }
    }
}

impl std::fmt::Display for PackageImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageImport(...)")
    }
}

