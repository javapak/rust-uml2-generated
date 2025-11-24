// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           TemplateSignature (struct)
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
use crate::template_parameter::TemplateParameter;
use crate::templateable_element::TemplateableElement;

lazy_static! {
    static ref TEMPLATE_SIGNATURE_REGISTRY: Mutex<RefCell<HashMap<String, TemplateSignature>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct TemplateSignature {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub parameter: Vec<String>,
    pub owned_parameter: Vec<String>,
    pub template: String,
}

#[wasm_bindgen]
impl TemplateSignature {
    /// Creates a new TemplateSignature and returns its ID
    #[wasm_bindgen]
    pub fn create(template: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            parameter: Vec::new(),
            owned_parameter: Vec::new(),
            template: template,
        };

        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a TemplateSignature by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a TemplateSignature instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: TemplateSignature = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a TemplateSignature by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a TemplateSignature exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all TemplateSignature instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<TemplateSignature> = TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of TemplateSignature instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all TemplateSignature instances
    #[wasm_bindgen]
    pub fn clear_all() {
        TEMPLATE_SIGNATURE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Adds a EAnnotation to e_annotations
    #[wasm_bindgen]
    pub fn add_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_comment.len();

        instance.owned_comment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a TemplateParameter to parameter
    #[wasm_bindgen]
    pub fn add_parameter(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.parameter.contains(&ref_id) {
            return Ok(false);
        }

        instance.parameter.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a TemplateParameter from parameter
    #[wasm_bindgen]
    pub fn remove_parameter(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.parameter.iter().position(|x| x == &ref_id) {
            instance.parameter.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all TemplateParameter from parameter
    #[wasm_bindgen]
    pub fn clear_parameter(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.parameter.len();

        instance.parameter.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a TemplateParameter to owned_parameter
    #[wasm_bindgen]
    pub fn add_owned_parameter(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_parameter.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_parameter.push(ref_id.clone());

        if let Ok(target_js) = TemplateParameter::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                target.signature = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateParameter::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a TemplateParameter from owned_parameter
    #[wasm_bindgen]
    pub fn remove_owned_parameter(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_parameter.iter().position(|x| x == &ref_id) {
            instance.owned_parameter.remove(pos);

        if let Ok(target_js) = TemplateParameter::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateParameter::update(target_js);
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

    /// Clears all TemplateParameter from owned_parameter
    #[wasm_bindgen]
    pub fn clear_owned_parameter(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_parameter.len();

        // Update all opposite references
        for ref_id in &instance.owned_parameter {
            if let Ok(target_js) = TemplateParameter::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateParameter::update(target_js);
                }
            }
        }
        }

        instance.owned_parameter.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the template reference
    #[wasm_bindgen]
    pub fn set_template(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: TemplateSignature = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Ok(target_js) = TemplateableElement::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateableElement>(target_js) {
                target.owned_template_signature = Some(instance_id.clone());
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateableElement::update(target_js);
                }
            }
        }

        instance.template = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl TemplateSignature {
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

        // Validate all parameter references exist
        for id in &self.parameter {
            if !TemplateParameter::exists(id.clone()) {
                errors.push(format!("TemplateParameter {} not found", id));
            }
        }

        // Validate all owned_parameter references exist
        for id in &self.owned_parameter {
            if !TemplateParameter::exists(id.clone()) {
                errors.push(format!("TemplateParameter {} not found", id));
            }
        }

        // Validate template reference exists
        if !TemplateableElement::exists(self.template.clone()) {
            errors.push(format!("TemplateableElement {} not found", self.template));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

