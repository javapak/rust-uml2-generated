use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::profile::Profile;
use crate::package::Package;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileApplication {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    applied_profile: Rc<RefCell<Profile>>,
    is_strict: bool,
    applying_package: Weak<RefCell<Package>>,
}

impl ProfileApplication {
    pub fn new(applied_profile: Rc<RefCell<Profile>>, is_strict: bool, applying_package: Weak<RefCell<Package>>) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            applied_profile: applied_profile,
            is_strict: is_strict,
            applying_package: applying_package,
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

    /// Returns a reference to applied_profile
    pub fn applied_profile(&self) -> &Rc<RefCell<Profile>> {
        &self.applied_profile
    }

    /// Returns a mutable reference to applied_profile
    pub fn applied_profile_mut(&mut self) -> &mut Rc<RefCell<Profile>> {
        &mut self.applied_profile
    }

    /// Sets applied_profile
    pub fn set_applied_profile(&mut self, value: Rc<RefCell<Profile>>) {
        self.applied_profile = value;
    }

    /// Returns is_strict
    pub fn is_strict(&self) -> bool {
        self.is_strict
    }

    /// Sets is_strict
    pub fn set_is_strict(&mut self, value: bool) {
        self.is_strict = value;
    }

    /// Returns a reference to applying_package
    pub fn applying_package(&self) -> &Weak<RefCell<Package>> {
        &self.applying_package
    }

    /// Returns a mutable reference to applying_package
    pub fn applying_package_mut(&mut self) -> &mut Weak<RefCell<Package>> {
        &mut self.applying_package
    }

    /// Sets applying_package
    pub fn set_applying_package(&mut self, value: Weak<RefCell<Package>>) {
        self.applying_package = value;
    }

}

impl Default for ProfileApplication {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            applied_profile: Default::default(),
            is_strict: false,
            applying_package: Default::default(),
        }
    }
}

impl std::fmt::Display for ProfileApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProfileApplication(...)")
    }
}

