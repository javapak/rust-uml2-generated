// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Generalization (struct)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-24 11:19:15
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
use crate::comment::Comment;
use crate::classifier::Classifier;
use crate::generalization_set::GeneralizationSet;

lazy_static! {
    static ref GENERALIZATION_REGISTRY: Mutex<RefCell<HashMap<String, Generalization>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct Generalization {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub is_substitutable: Option<String>,
    pub general: String,
    pub generalization_set: Vec<String>,
    pub specific: String,
}

#[wasm_bindgen]
impl Generalization {
    /// Creates a new Generalization and returns its ID
    #[wasm_bindgen]
    pub fn create(general: String, specific: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            is_substitutable: None,
            general: general,
            generalization_set: Vec::new(),
            specific: specific,
        };

        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a Generalization by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a Generalization instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: Generalization = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a Generalization by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a Generalization exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all Generalization instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<Generalization> = GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of Generalization instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all Generalization instances
    #[wasm_bindgen]
    pub fn clear_all() {
        GENERALIZATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Adds a EAnnotation to e_annotations
    #[wasm_bindgen]
    pub fn add_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.e_annotations.len();

        instance.e_annotations.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Comment to owned_comment
    #[wasm_bindgen]
    pub fn add_owned_comment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_comment.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_comment.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Comment from owned_comment
    #[wasm_bindgen]
    pub fn remove_owned_comment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_comment.iter().position(|x| x == &ref_id) {
            instance.owned_comment.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Comment from owned_comment
    #[wasm_bindgen]
    pub fn clear_owned_comment(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_comment.len();

        instance.owned_comment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the general reference
    #[wasm_bindgen]
    pub fn set_general(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.general = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a GeneralizationSet to generalization_set
    #[wasm_bindgen]
    pub fn add_generalization_set(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.generalization_set.contains(&ref_id) {
            return Ok(false);
        }

        instance.generalization_set.push(ref_id.clone());

        if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                if !target.generalization.contains(&instance_id) {
                    target.generalization.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = GeneralizationSet::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a GeneralizationSet from generalization_set
    #[wasm_bindgen]
    pub fn remove_generalization_set(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.generalization_set.iter().position(|x| x == &ref_id) {
            instance.generalization_set.remove(pos);

        if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                target.generalization.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = GeneralizationSet::update(target_js);
                }
            }
        }

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all GeneralizationSet from generalization_set
    #[wasm_bindgen]
    pub fn clear_generalization_set(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.generalization_set.len();

        // Update all opposite references
        for ref_id in &instance.generalization_set {
            if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                target.generalization.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = GeneralizationSet::update(target_js);
                }
            }
        }
        }

        instance.generalization_set.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the specific reference
    #[wasm_bindgen]
    pub fn set_specific(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Ok(target_js) = Classifier::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Classifier>(target_js) {
                if !target.generalization.contains(&instance_id) {
                    target.generalization.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Classifier::update(target_js);
                }
            }
        }

        instance.specific = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_substitutable field
    #[wasm_bindgen]
    pub fn set_is_substitutable(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Generalization = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_substitutable = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl Generalization {
    /// Validates this instance and all references
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate all e_annotations references exist
        for id in &self.e_annotations {
            if !EAnnotation::exists(id.clone()) {
                errors.push(format!("EAnnotation {} not found", id));
            }
        }

        // Validate all owned_comment references exist
        for id in &self.owned_comment {
            if !Comment::exists(id.clone()) {
                errors.push(format!("Comment {} not found", id));
            }
        }

        // Validate general reference exists
        if !Classifier::exists(self.general.clone()) {
            errors.push(format!("Classifier {} not found", self.general));
        }

        // Validate all generalization_set references exist
        for id in &self.generalization_set {
            if !GeneralizationSet::exists(id.clone()) {
                errors.push(format!("GeneralizationSet {} not found", id));
            }
        }

        // Validate specific reference exists
        if !Classifier::exists(self.specific.clone()) {
            errors.push(format!("Classifier {} not found", self.specific));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

