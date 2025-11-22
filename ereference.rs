// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EReference (struct)
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
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct EReference {
    e_annotations: Vec<EAnnotation>,
    name: Option<String>,
    ordered: Option<bool>,
    unique: Option<bool>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    e_type: Option<String>,
    changeable: Option<bool>,
    volatile: Option<bool>,
    transient: Option<bool>,
    default_value_literal: Option<String>,
    unsettable: Option<bool>,
    derived: Option<bool>,
    containment: Option<bool>,
    resolve_proxies: Option<bool>,
    e_opposite: Option<String>,
    e_keys: Vec<String>,
}

#[wasm_bindgen]
impl EReference {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            name: None,
            ordered: None,
            unique: None,
            lower_bound: None,
            upper_bound: None,
            e_type: None,
            changeable: None,
            volatile: None,
            transient: None,
            default_value_literal: None,
            unsettable: None,
            derived: None,
            containment: None,
            resolve_proxies: None,
            e_opposite: None,
            e_keys: Vec::new(),
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

    /// Returns a clone of ordered if present
    pub fn ordered(&self) -> Option<bool> {
        self.ordered.clone()
    }

    /// Sets ordered
    pub fn set_ordered(&mut self, value: bool) {
        self.ordered = Some(value);
    }

    /// Takes ordered, leaving None in its place
    pub fn take_ordered(&mut self) -> Option<bool> {
        self.ordered.take()
    }

    /// Returns a clone of unique if present
    pub fn unique(&self) -> Option<bool> {
        self.unique.clone()
    }

    /// Sets unique
    pub fn set_unique(&mut self, value: bool) {
        self.unique = Some(value);
    }

    /// Takes unique, leaving None in its place
    pub fn take_unique(&mut self) -> Option<bool> {
        self.unique.take()
    }

    /// Returns a clone of lower_bound if present
    pub fn lower_bound(&self) -> Option<i32> {
        self.lower_bound.clone()
    }

    /// Sets lower_bound
    pub fn set_lower_bound(&mut self, value: i32) {
        self.lower_bound = Some(value);
    }

    /// Takes lower_bound, leaving None in its place
    pub fn take_lower_bound(&mut self) -> Option<i32> {
        self.lower_bound.take()
    }

    /// Returns a clone of upper_bound if present
    pub fn upper_bound(&self) -> Option<i32> {
        self.upper_bound.clone()
    }

    /// Sets upper_bound
    pub fn set_upper_bound(&mut self, value: i32) {
        self.upper_bound = Some(value);
    }

    /// Takes upper_bound, leaving None in its place
    pub fn take_upper_bound(&mut self) -> Option<i32> {
        self.upper_bound.take()
    }

    /// Returns a clone of e_type if present
    pub fn e_type(&self) -> Option<String> {
        self.e_type.clone()
    }

    /// Sets e_type
    pub fn set_e_type(&mut self, value: String) {
        self.e_type = Some(value);
    }

    /// Takes e_type, leaving None in its place
    pub fn take_e_type(&mut self) -> Option<String> {
        self.e_type.take()
    }

    /// Returns a clone of changeable if present
    pub fn changeable(&self) -> Option<bool> {
        self.changeable.clone()
    }

    /// Sets changeable
    pub fn set_changeable(&mut self, value: bool) {
        self.changeable = Some(value);
    }

    /// Takes changeable, leaving None in its place
    pub fn take_changeable(&mut self) -> Option<bool> {
        self.changeable.take()
    }

    /// Returns a clone of volatile if present
    pub fn volatile(&self) -> Option<bool> {
        self.volatile.clone()
    }

    /// Sets volatile
    pub fn set_volatile(&mut self, value: bool) {
        self.volatile = Some(value);
    }

    /// Takes volatile, leaving None in its place
    pub fn take_volatile(&mut self) -> Option<bool> {
        self.volatile.take()
    }

    /// Returns a clone of transient if present
    pub fn transient(&self) -> Option<bool> {
        self.transient.clone()
    }

    /// Sets transient
    pub fn set_transient(&mut self, value: bool) {
        self.transient = Some(value);
    }

    /// Takes transient, leaving None in its place
    pub fn take_transient(&mut self) -> Option<bool> {
        self.transient.take()
    }

    /// Returns a clone of default_value_literal if present
    pub fn default_value_literal(&self) -> Option<String> {
        self.default_value_literal.clone()
    }

    /// Sets default_value_literal
    pub fn set_default_value_literal(&mut self, value: String) {
        self.default_value_literal = Some(value);
    }

    /// Takes default_value_literal, leaving None in its place
    pub fn take_default_value_literal(&mut self) -> Option<String> {
        self.default_value_literal.take()
    }

    /// Returns a clone of unsettable if present
    pub fn unsettable(&self) -> Option<bool> {
        self.unsettable.clone()
    }

    /// Sets unsettable
    pub fn set_unsettable(&mut self, value: bool) {
        self.unsettable = Some(value);
    }

    /// Takes unsettable, leaving None in its place
    pub fn take_unsettable(&mut self) -> Option<bool> {
        self.unsettable.take()
    }

    /// Returns a clone of derived if present
    pub fn derived(&self) -> Option<bool> {
        self.derived.clone()
    }

    /// Sets derived
    pub fn set_derived(&mut self, value: bool) {
        self.derived = Some(value);
    }

    /// Takes derived, leaving None in its place
    pub fn take_derived(&mut self) -> Option<bool> {
        self.derived.take()
    }

    /// Returns a clone of containment if present
    pub fn containment(&self) -> Option<bool> {
        self.containment.clone()
    }

    /// Sets containment
    pub fn set_containment(&mut self, value: bool) {
        self.containment = Some(value);
    }

    /// Takes containment, leaving None in its place
    pub fn take_containment(&mut self) -> Option<bool> {
        self.containment.take()
    }

    /// Returns a clone of resolve_proxies if present
    pub fn resolve_proxies(&self) -> Option<bool> {
        self.resolve_proxies.clone()
    }

    /// Sets resolve_proxies
    pub fn set_resolve_proxies(&mut self, value: bool) {
        self.resolve_proxies = Some(value);
    }

    /// Takes resolve_proxies, leaving None in its place
    pub fn take_resolve_proxies(&mut self) -> Option<bool> {
        self.resolve_proxies.take()
    }

    /// Returns a clone of e_opposite if present
    pub fn e_opposite(&self) -> Option<String> {
        self.e_opposite.clone()
    }

    /// Sets e_opposite
    pub fn set_e_opposite(&mut self, value: String) {
        self.e_opposite = Some(value);
    }

    /// Takes e_opposite, leaving None in its place
    pub fn take_e_opposite(&mut self) -> Option<String> {
        self.e_opposite.take()
    }

    /// Returns a clone of e_keys
    pub fn e_keys(&self) -> Vec<String> {
        self.e_keys.clone()
    }

    /// Adds an existing EAttribute to e_keys by ID
    pub fn add_e_key_by_id(&mut self, id: String) {
        self.e_keys.push(id);
    }

    /// Clears all items from e_keys
    pub fn clear_e_keys(&mut self) {
        self.e_keys.clear();
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
        "EReference".to_string()
    }

}

impl std::fmt::Display for EReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "<none>")
        }
    }
}

