// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ProfileApplication (struct)
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
use crate::profile::Profile;
use crate::package::Package;

lazy_static! {
    static ref PROFILE_APPLICATION_REGISTRY: Mutex<RefCell<HashMap<String, ProfileApplication>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct ProfileApplication {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub applied_profile: String,
    pub is_strict: String,
    pub applying_package: String,
}

#[wasm_bindgen]
impl ProfileApplication {
    /// Creates a new ProfileApplication and returns its ID
    #[wasm_bindgen]
    pub fn create(applied_profile: String, is_strict: String, applying_package: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            applied_profile: applied_profile,
            is_strict: is_strict,
            applying_package: applying_package,
        };

        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a ProfileApplication by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a ProfileApplication instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: ProfileApplication = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a ProfileApplication by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a ProfileApplication exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all ProfileApplication instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<ProfileApplication> = PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of ProfileApplication instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all ProfileApplication instances
    #[wasm_bindgen]
    pub fn clear_all() {
        PROFILE_APPLICATION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Adds a EAnnotation to e_annotations
    #[wasm_bindgen]
    pub fn add_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_comment.len();

        instance.owned_comment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the applied_profile reference
    #[wasm_bindgen]
    pub fn set_applied_profile(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.applied_profile = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the applying_package reference
    #[wasm_bindgen]
    pub fn set_applying_package(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Ok(target_js) = Package::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Package>(target_js) {
                if !target.profile_application.contains(&instance_id) {
                    target.profile_application.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Package::update(target_js);
                }
            }
        }

        instance.applying_package = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_strict field
    #[wasm_bindgen]
    pub fn set_is_strict(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ProfileApplication = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_strict = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl ProfileApplication {
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

        // Validate applied_profile reference exists
        if !Profile::exists(self.applied_profile.clone()) {
            errors.push(format!("Profile {} not found", self.applied_profile));
        }

        // Validate applying_package reference exists
        if !Package::exists(self.applying_package.clone()) {
            errors.push(format!("Package {} not found", self.applying_package));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

