use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::package::Package;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMerge {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    merged_package: Rc<RefCell<Package>>,
    receiving_package: Weak<RefCell<Package>>,
}

impl PackageMerge {
    pub fn new(merged_package: Rc<RefCell<Package>>, receiving_package: Weak<RefCell<Package>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            merged_package: merged_package,
            receiving_package: receiving_package,
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

    /// Returns a reference to merged_package
    pub fn merged_package(&self) -> &Rc<RefCell<Package>> {
        &self.merged_package
    }

    /// Returns a mutable reference to merged_package
    pub fn merged_package_mut(&mut self) -> &mut Rc<RefCell<Package>> {
        &mut self.merged_package
    }

    /// Sets merged_package
    pub fn set_merged_package(&mut self, value: Rc<RefCell<Package>>) {
        self.merged_package = value;
    }

    /// Returns a reference to receiving_package
    pub fn receiving_package(&self) -> &Weak<RefCell<Package>> {
        &self.receiving_package
    }

    /// Returns a mutable reference to receiving_package
    pub fn receiving_package_mut(&mut self) -> &mut Weak<RefCell<Package>> {
        &mut self.receiving_package
    }

    /// Sets receiving_package
    pub fn set_receiving_package(&mut self, value: Weak<RefCell<Package>>) {
        self.receiving_package = value;
    }

}

impl Default for PackageMerge {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            merged_package: Default::default(),
            receiving_package: Default::default(),
        }
    }
}

impl std::fmt::Display for PackageMerge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageMerge(...)")
    }
}

