use crate::eannotation::EAnnotation;
use crate::eclassifier::EClassifier;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EPackage {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ns_uri: Option<String>,
    ns_prefix: Option<String>,
    e_classifiers: Vec<EClassifier>,
    e_subpackages: Vec<Box<EPackage>>,
}

impl EPackage {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ns_uri: None,
            ns_prefix: None,
            e_classifiers: Vec::new(),
            e_subpackages: Vec::new(),
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

    /// Returns a reference to name if present
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns a mutable reference to name if present
    pub fn name_mut(&mut self) -> Option<&mut String> {
        self.name.as_mut()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a reference to ns_uri if present
    pub fn ns_uri(&self) -> Option<&String> {
        self.ns_uri.as_ref()
    }

    /// Returns a mutable reference to ns_uri if present
    pub fn ns_uri_mut(&mut self) -> Option<&mut String> {
        self.ns_uri.as_mut()
    }

    /// Sets ns_uri
    pub fn set_ns_uri(&mut self, value: String) {
        self.ns_uri = Some(value);
    }

    /// Takes ns_uri, leaving None in its place
    pub fn take_ns_uri(&mut self) -> Option<String> {
        self.ns_uri.take()
    }

    /// Returns a reference to ns_prefix if present
    pub fn ns_prefix(&self) -> Option<&String> {
        self.ns_prefix.as_ref()
    }

    /// Returns a mutable reference to ns_prefix if present
    pub fn ns_prefix_mut(&mut self) -> Option<&mut String> {
        self.ns_prefix.as_mut()
    }

    /// Sets ns_prefix
    pub fn set_ns_prefix(&mut self, value: String) {
        self.ns_prefix = Some(value);
    }

    /// Takes ns_prefix, leaving None in its place
    pub fn take_ns_prefix(&mut self) -> Option<String> {
        self.ns_prefix.take()
    }

    /// Returns a slice of e_classifiers
    pub fn e_classifiers(&self) -> &[EClassifier] {
        &self.e_classifiers
    }

    /// Returns a mutable reference to e_classifiers
    pub fn e_classifiers_mut(&mut self) -> &mut Vec<EClassifier> {
        &mut self.e_classifiers
    }

    /// Adds an item to e_classifiers
    pub fn add_e_classifier(&mut self, item: EClassifier) {
        self.e_classifiers.push(item);
    }

    /// Clears all items from e_classifiers
    pub fn clear_e_classifiers(&mut self) {
        self.e_classifiers.clear();
    }

    /// Returns a reference to e_subpackages
    pub fn e_subpackages(&self) -> &Vec<Box<EPackage>> {
        &self.e_subpackages
    }

    /// Returns a mutable reference to e_subpackages
    pub fn e_subpackages_mut(&mut self) -> &mut Vec<Box<EPackage>> {
        &mut self.e_subpackages
    }

    /// Adds an item to e_subpackages
    pub fn add_e_subpackage(&mut self, item: EPackage) {
        self.e_subpackages.push(Box::new(item));
    }

    /// Clears all items from e_subpackages
    pub fn clear_e_subpackages(&mut self) {
        self.e_subpackages.clear();
    }

}

impl std::fmt::Display for EPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

