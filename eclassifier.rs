use crate::eannotation::EAnnotation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EClassifier {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    instance_class_name: Option<String>,
    instance_type_name: Option<String>,
}

impl EClassifier {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            instance_class_name: None,
            instance_type_name: None,
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

    /// Returns a reference to instance_class_name if present
    pub fn instance_class_name(&self) -> Option<&String> {
        self.instance_class_name.as_ref()
    }

    /// Returns a mutable reference to instance_class_name if present
    pub fn instance_class_name_mut(&mut self) -> Option<&mut String> {
        self.instance_class_name.as_mut()
    }

    /// Sets instance_class_name
    pub fn set_instance_class_name(&mut self, value: String) {
        self.instance_class_name = Some(value);
    }

    /// Takes instance_class_name, leaving None in its place
    pub fn take_instance_class_name(&mut self) -> Option<String> {
        self.instance_class_name.take()
    }

    /// Returns a reference to instance_type_name if present
    pub fn instance_type_name(&self) -> Option<&String> {
        self.instance_type_name.as_ref()
    }

    /// Returns a mutable reference to instance_type_name if present
    pub fn instance_type_name_mut(&mut self) -> Option<&mut String> {
        self.instance_type_name.as_mut()
    }

    /// Sets instance_type_name
    pub fn set_instance_type_name(&mut self, value: String) {
        self.instance_type_name = Some(value);
    }

    /// Takes instance_type_name, leaving None in its place
    pub fn take_instance_type_name(&mut self) -> Option<String> {
        self.instance_type_name.take()
    }

}

