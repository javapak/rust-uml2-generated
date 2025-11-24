// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EParameter (struct)
// Source Package: ecore
// Package URI:    http://www.eclipse.org/emf/2002/Ecore
// Generated:      2025-11-24 09:27:25
// Generator:      EcoreToRustGenerator v0.1.0
//
// Generation Options:
//   - WASM:       enabled
//   - Tsify:      enabled
//   - Serde:      enabled
//   - Builders:   disabled
//   - References: String IDs
//
// WARNING: This file is auto-generated. Manual changes will be overwritten.
// ============================================================================

use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::eannotation::EAnnotation;
use crate::eclassifier::EClassifier;

lazy_static! {
    static ref E_PARAMETER_REGISTRY: Mutex<RefCell<HashMap<String, EParameter>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct EParameter {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub name: Option<String>,
    pub ordered: Option<bool>,
    pub unique: Option<bool>,
    pub lower_bound: Option<i32>,
    pub upper_bound: Option<i32>,
    pub e_type: Option<String>,
}

#[wasm_bindgen]
impl EParameter {
    /// Creates a new EParameter and returns its ID
    #[wasm_bindgen]
    pub fn create() -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            name: None,
            ordered: None,
            unique: None,
            lower_bound: None,
            upper_bound: None,
            e_type: None,
        };

        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a EParameter by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a EParameter instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: EParameter = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a EParameter by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a EParameter exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all EParameter instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<EParameter> = E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of EParameter instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all EParameter instances
    #[wasm_bindgen]
    pub fn clear_all() {
        E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds EParameter instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<EParameter> = E_PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .filter(|item| {
                item.name.as_ref()
                    .map(|n| n.contains(&pattern))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Adds a EAnnotation to e_annotations
    #[wasm_bindgen]
    pub fn add_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.e_annotations.contains(&ref_id) {
            return Ok(false);
        }

        instance.e_annotations.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a EAnnotation from e_annotations
    #[wasm_bindgen]
    pub fn remove_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.e_annotations.iter().position(|x| x == &ref_id) {
            instance.e_annotations.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all EAnnotation from e_annotations
    #[wasm_bindgen]
    pub fn clear_e_annotations(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.e_annotations.len();

        instance.e_annotations.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the e_type reference
    #[wasm_bindgen]
    pub fn set_e_type(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(new_ref_id) = &ref_id {
        }

        instance.e_type = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Clears the e_type reference
    #[wasm_bindgen]
    pub fn clear_e_type(instance_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.e_type.is_none() {
            return Ok(false);
        }

        if let Some(old_ref_id) = &instance.e_type {
        }

        instance.e_type = None;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the ordered field
    #[wasm_bindgen]
    pub fn set_ordered(instance_id: String, value: Option<bool>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.ordered = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the unique field
    #[wasm_bindgen]
    pub fn set_unique(instance_id: String, value: Option<bool>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.unique = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the lower_bound field
    #[wasm_bindgen]
    pub fn set_lower_bound(instance_id: String, value: Option<i32>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.lower_bound = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the upper_bound field
    #[wasm_bindgen]
    pub fn set_upper_bound(instance_id: String, value: Option<i32>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EParameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.upper_bound = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl EParameter {
    /// Validates this instance and all references
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate all e_annotations references exist
        for id in &self.e_annotations {
            if !EAnnotation::exists(id.clone()) {
                errors.push(format!("EAnnotation {} not found", id));
            }
        }

        // Validate e_type reference exists
        if let Some(id) = &self.e_type {
            if !EClassifier::exists(id.clone()) {
                errors.push(format!("EClassifier {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

