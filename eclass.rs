// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EClass (struct)
// Source Package: ecore
// Package URI:    http://www.eclipse.org/emf/2002/Ecore
// Generated:      2025-11-22 12:14:07
// Generator:      EcoreToRustGenerator v0.1.0
//
// Generation Options:
//   - WASM:       enabled
//   - Tsify:      disabled
//   - Serde:      enabled
//   - Builders:   disabled
//   - References: String IDs
//
// WARNING: This file is auto-generated. Manual changes will be overwritten.
// ============================================================================

use crate::eannotation::EAnnotation;
use crate::eoperation::EOperation;
use crate::estructural_feature::EStructuralFeature;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EClass {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    instance_class_name: Option<String>,
    instance_type_name: Option<String>,
    abstract_: Option<bool>,
    interface: Option<bool>,
    e_super_types: Vec<String>,
    e_operations: Vec<EOperation>,
    e_structural_features: Vec<EStructuralFeature>,
}

#[wasm_bindgen]
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

    /// Returns a clone of name if present
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Sets name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Takes name, leaving None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }

    /// Returns a clone of instance_class_name if present
    pub fn instance_class_name(&self) -> Option<String> {
        self.instance_class_name.clone()
    }

    /// Sets instance_class_name
    pub fn set_instance_class_name(&mut self, value: String) {
        self.instance_class_name = Some(value);
    }

    /// Takes instance_class_name, leaving None in its place
    pub fn take_instance_class_name(&mut self) -> Option<String> {
        self.instance_class_name.take()
    }

    /// Returns a clone of instance_type_name if present
    pub fn instance_type_name(&self) -> Option<String> {
        self.instance_type_name.clone()
    }

    /// Sets instance_type_name
    pub fn set_instance_type_name(&mut self, value: String) {
        self.instance_type_name = Some(value);
    }

    /// Takes instance_type_name, leaving None in its place
    pub fn take_instance_type_name(&mut self) -> Option<String> {
        self.instance_type_name.take()
    }

    /// Returns a clone of abstract_ if present
    pub fn abstract_(&self) -> Option<bool> {
        self.abstract_.clone()
    }

    /// Sets abstract_
    pub fn set_abstract_(&mut self, value: bool) {
        self.abstract_ = Some(value);
    }

    /// Takes abstract_, leaving None in its place
    pub fn take_abstract(&mut self) -> Option<bool> {
        self.abstract_.take()
    }

    /// Returns a clone of interface if present
    pub fn interface(&self) -> Option<bool> {
        self.interface.clone()
    }

    /// Sets interface
    pub fn set_interface(&mut self, value: bool) {
        self.interface = Some(value);
    }

    /// Takes interface, leaving None in its place
    pub fn take_interface(&mut self) -> Option<bool> {
        self.interface.take()
    }

    /// Returns a clone of e_super_types
    pub fn e_super_types(&self) -> Vec<String> {
        self.e_super_types.clone()
    }

    /// Adds an existing EClass to e_super_types by ID
    pub fn add_e_super_type_by_id(&mut self, id: String) {
        self.e_super_types.push(id);
    }

    /// Clears all items from e_super_types
    pub fn clear_e_super_types(&mut self) {
        self.e_super_types.clear();
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|e| e.to_string())
    }

    /// Deserialize from JSON string
    pub fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
    }

    /// Returns whether this type can be created standalone (not nested)
    pub fn can_exist_standalone() -> bool {
        true
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        false
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "EClass".to_string()
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

