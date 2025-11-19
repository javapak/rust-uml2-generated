use std::rc::Weak;
use crate::eannotation::EAnnotation;
use crate::eoperation::EOperation;
use crate::estructural_feature::EStructuralFeature;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EClass {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    instance_class_name: Option<String>,
    instance_type_name: Option<String>,
    abstract_: Option<bool>,
    interface: Option<bool>,
    e_super_types: Vec<Rc<RefCell<EClass>>>,
    e_operations: Vec<EOperation>,
    e_structural_features: Vec<EStructuralFeature>,
}

impl EClass {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            instance_class_name: None,
            instance_type_name: None,
            abstract_: None,
            interface: None,
            e_super_types: Vec::new(),
            e_operations: Vec::new(),
            e_structural_features: Vec::new(),
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

    /// Returns a reference to abstract_ if present
    pub fn abstract_(&self) -> Option<&bool> {
        self.abstract_.as_ref()
    }

    /// Returns a mutable reference to abstract_ if present
    pub fn abstract_mut(&mut self) -> Option<&mut bool> {
        self.abstract_.as_mut()
    }

    /// Sets abstract_
    pub fn set_abstract_(&mut self, value: bool) {
        self.abstract_ = Some(value);
    }

    /// Takes abstract_, leaving None in its place
    pub fn take_abstract(&mut self) -> Option<bool> {
        self.abstract_.take()
    }

    /// Returns a reference to interface if present
    pub fn interface(&self) -> Option<&bool> {
        self.interface.as_ref()
    }

    /// Returns a mutable reference to interface if present
    pub fn interface_mut(&mut self) -> Option<&mut bool> {
        self.interface.as_mut()
    }

    /// Sets interface
    pub fn set_interface(&mut self, value: bool) {
        self.interface = Some(value);
    }

    /// Takes interface, leaving None in its place
    pub fn take_interface(&mut self) -> Option<bool> {
        self.interface.take()
    }

    /// Returns a reference to e_super_types
    pub fn e_super_types(&self) -> &Vec<Rc<RefCell<EClass>>> {
        &self.e_super_types
    }

    /// Returns a mutable reference to e_super_types
    pub fn e_super_types_mut(&mut self) -> &mut Vec<Rc<RefCell<EClass>>> {
        &mut self.e_super_types
    }

    /// Adds an item to e_super_types
    pub fn add_e_super_type(&mut self, item: Rc<RefCell<EClass>>) {
        self.e_super_types.push(item);
    }

    /// Clears all items from e_super_types
    pub fn clear_e_super_types(&mut self) {
        self.e_super_types.clear();
    }

    /// Returns a slice of e_operations
    pub fn e_operations(&self) -> &[EOperation] {
        &self.e_operations
    }

    /// Returns a mutable reference to e_operations
    pub fn e_operations_mut(&mut self) -> &mut Vec<EOperation> {
        &mut self.e_operations
    }

    /// Adds an item to e_operations
    pub fn add_e_operation(&mut self, item: EOperation) {
        self.e_operations.push(item);
    }

    /// Clears all items from e_operations
    pub fn clear_e_operations(&mut self) {
        self.e_operations.clear();
    }

    /// Returns a slice of e_structural_features
    pub fn e_structural_features(&self) -> &[EStructuralFeature] {
        &self.e_structural_features
    }

    /// Returns a mutable reference to e_structural_features
    pub fn e_structural_features_mut(&mut self) -> &mut Vec<EStructuralFeature> {
        &mut self.e_structural_features
    }

    /// Adds an item to e_structural_features
    pub fn add_e_structural_feature(&mut self, item: EStructuralFeature) {
        self.e_structural_features.push(item);
    }

    /// Clears all items from e_structural_features
    pub fn clear_e_structural_features(&mut self) {
        self.e_structural_features.clear();
    }

}

impl std::fmt::Display for EClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

