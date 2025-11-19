use crate::eannotation::EAnnotation;
use crate::eenum_literal::EEnumLiteral;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EEnum {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    instance_class_name: Option<String>,
    instance_type_name: Option<String>,
    serializable: Option<bool>,
    e_literals: Vec<EEnumLiteral>,
}

impl EEnum {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            instance_class_name: None,
            instance_type_name: None,
            serializable: None,
            e_literals: Vec::new(),
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

    /// Returns a reference to serializable if present
    pub fn serializable(&self) -> Option<&bool> {
        self.serializable.as_ref()
    }

    /// Returns a mutable reference to serializable if present
    pub fn serializable_mut(&mut self) -> Option<&mut bool> {
        self.serializable.as_mut()
    }

    /// Sets serializable
    pub fn set_serializable(&mut self, value: bool) {
        self.serializable = Some(value);
    }

    /// Takes serializable, leaving None in its place
    pub fn take_serializable(&mut self) -> Option<bool> {
        self.serializable.take()
    }

    /// Returns a slice of e_literals
    pub fn e_literals(&self) -> &[EEnumLiteral] {
        &self.e_literals
    }

    /// Returns a mutable reference to e_literals
    pub fn e_literals_mut(&mut self) -> &mut Vec<EEnumLiteral> {
        &mut self.e_literals
    }

    /// Adds an item to e_literals
    pub fn add_e_literal(&mut self, item: EEnumLiteral) {
        self.e_literals.push(item);
    }

    /// Clears all items from e_literals
    pub fn clear_e_literals(&mut self) {
        self.e_literals.clear();
    }

}

impl std::fmt::Display for EEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

