use crate::eannotation::EAnnotation;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EEnumLiteral {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    value: Option<i32>,
    literal: Option<String>,
}

impl EEnumLiteral {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            value: None,
            literal: None,
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

    /// Returns a reference to value if present
    pub fn value(&self) -> Option<&i32> {
        self.value.as_ref()
    }

    /// Returns a mutable reference to value if present
    pub fn value_mut(&mut self) -> Option<&mut i32> {
        self.value.as_mut()
    }

    /// Sets value
    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    /// Takes value, leaving None in its place
    pub fn take_value(&mut self) -> Option<i32> {
        self.value.take()
    }

    /// Returns a reference to literal if present
    pub fn literal(&self) -> Option<&String> {
        self.literal.as_ref()
    }

    /// Returns a mutable reference to literal if present
    pub fn literal_mut(&mut self) -> Option<&mut String> {
        self.literal.as_mut()
    }

    /// Sets literal
    pub fn set_literal(&mut self, value: String) {
        self.literal = Some(value);
    }

    /// Takes literal, leaving None in its place
    pub fn take_literal(&mut self) -> Option<String> {
        self.literal.take()
    }

}

impl std::fmt::Display for EEnumLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

