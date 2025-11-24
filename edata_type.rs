// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           EDataType (struct)
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

lazy_static! {
    static ref E_DATA_TYPE_REGISTRY: Mutex<RefCell<HashMap<String, EDataType>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct EDataType {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub name: Option<String>,
    pub instance_class_name: Option<String>,
    pub instance_type_name: Option<String>,
    pub serializable: Option<bool>,
}

#[wasm_bindgen]
impl EDataType {
    /// Creates a new EDataType and returns its ID
    #[wasm_bindgen]
    pub fn create() -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            name: None,
            instance_class_name: None,
            instance_type_name: None,
            serializable: None,
        };

        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a EDataType by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a EDataType instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: EDataType = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a EDataType by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a EDataType exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all EDataType instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<EDataType> = E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of EDataType instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all EDataType instances
    #[wasm_bindgen]
    pub fn clear_all() {
        E_DATA_TYPE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds EDataType instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<EDataType> = E_DATA_TYPE_REGISTRY.lock().unwrap()
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
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.e_annotations.len();

        instance.e_annotations.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the instance_class_name field
    #[wasm_bindgen]
    pub fn set_instance_class_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.instance_class_name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the instance_type_name field
    #[wasm_bindgen]
    pub fn set_instance_type_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.instance_type_name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the serializable field
    #[wasm_bindgen]
    pub fn set_serializable(instance_id: String, value: Option<bool>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: EDataType = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.serializable = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl EDataType {
    /// Validates this instance and all references
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate all e_annotations references exist
        for id in &self.e_annotations {
            if !EAnnotation::exists(id.clone()) {
                errors.push(format!("EAnnotation {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

