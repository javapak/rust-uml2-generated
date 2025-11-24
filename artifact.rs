// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Artifact (struct)
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
use crate::visibility_kind::VisibilityKind;
use crate::dependency::Dependency;
use crate::string_expression::StringExpression;
use crate::element_import::ElementImport;
use crate::package_import::PackageImport;
use crate::constraint::Constraint;
use crate::template_parameter::TemplateParameter;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::generalization::Generalization;
use crate::generalization_set::GeneralizationSet;
use crate::classifier::Classifier;
use crate::substitution::Substitution;
use crate::collaboration_use::CollaborationUse;
use crate::use_case::UseCase;
use crate::manifestation::Manifestation;
use crate::operation::Operation;
use crate::property::Property;

lazy_static! {
    static ref ARTIFACT_REGISTRY: Mutex<RefCell<HashMap<String, Artifact>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct Artifact {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub name: Option<String>,
    pub visibility: Option<VisibilityKind>,
    pub client_dependency: Vec<String>,
    pub name_expression: Option<String>,
    pub element_import: Vec<String>,
    pub package_import: Vec<String>,
    pub owned_rule: Vec<String>,
    pub is_leaf: String,
    pub owning_template_parameter: Option<String>,
    pub template_parameter: Option<String>,
    pub template_binding: Vec<String>,
    pub owned_template_signature: Option<String>,
    pub is_abstract: String,
    pub generalization: Vec<String>,
    pub powertype_extent: Vec<String>,
    pub redefined_classifier: Vec<String>,
    pub substitution: Vec<String>,
    pub representation: Option<String>,
    pub collaboration_use: Vec<String>,
    pub owned_use_case: Vec<String>,
    pub use_case: Vec<String>,
    pub file_name: Option<String>,
    pub nested_artifact: Vec<String>,
    pub manifestation: Vec<String>,
    pub owned_operation: Vec<String>,
    pub owned_attribute: Vec<String>,
}

#[wasm_bindgen]
impl Artifact {
    /// Creates a new Artifact and returns its ID
    #[wasm_bindgen]
    pub fn create(is_leaf: String, is_abstract: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            element_import: Vec::new(),
            package_import: Vec::new(),
            owned_rule: Vec::new(),
            is_leaf: is_leaf,
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            is_abstract: is_abstract,
            generalization: Vec::new(),
            powertype_extent: Vec::new(),
            redefined_classifier: Vec::new(),
            substitution: Vec::new(),
            representation: None,
            collaboration_use: Vec::new(),
            owned_use_case: Vec::new(),
            use_case: Vec::new(),
            file_name: None,
            nested_artifact: Vec::new(),
            manifestation: Vec::new(),
            owned_operation: Vec::new(),
            owned_attribute: Vec::new(),
        };

        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a Artifact by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a Artifact instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: Artifact = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a Artifact by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a Artifact exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all Artifact instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<Artifact> = ARTIFACT_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of Artifact instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all Artifact instances
    #[wasm_bindgen]
    pub fn clear_all() {
        ARTIFACT_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds Artifact instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<Artifact> = ARTIFACT_REGISTRY.lock().unwrap()
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_comment.len();

        instance.owned_comment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Dependency to client_dependency
    #[wasm_bindgen]
    pub fn add_client_dependency(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.client_dependency.contains(&ref_id) {
            return Ok(false);
        }

        instance.client_dependency.push(ref_id.clone());

        if let Ok(target_js) = Dependency::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Dependency>(target_js) {
                if !target.client.contains(&instance_id) {
                    target.client.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Dependency::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Dependency from client_dependency
    #[wasm_bindgen]
    pub fn remove_client_dependency(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.client_dependency.iter().position(|x| x == &ref_id) {
            instance.client_dependency.remove(pos);

        if let Ok(target_js) = Dependency::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Dependency>(target_js) {
                target.client.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Dependency::update(target_js);
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

    /// Clears all Dependency from client_dependency
    #[wasm_bindgen]
    pub fn clear_client_dependency(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.client_dependency.len();

        // Update all opposite references
        for ref_id in &instance.client_dependency {
            if let Ok(target_js) = Dependency::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Dependency>(target_js) {
                target.client.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Dependency::update(target_js);
                }
            }
        }
        }

        instance.client_dependency.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the name_expression reference
    #[wasm_bindgen]
    pub fn set_name_expression(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name_expression = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a ElementImport to element_import
    #[wasm_bindgen]
    pub fn add_element_import(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.element_import.contains(&ref_id) {
            return Ok(false);
        }

        instance.element_import.push(ref_id.clone());

        if let Ok(target_js) = ElementImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ElementImport>(target_js) {
                target.importing_namespace = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ElementImport::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ElementImport from element_import
    #[wasm_bindgen]
    pub fn remove_element_import(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.element_import.iter().position(|x| x == &ref_id) {
            instance.element_import.remove(pos);

        if let Ok(target_js) = ElementImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ElementImport>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ElementImport::update(target_js);
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

    /// Clears all ElementImport from element_import
    #[wasm_bindgen]
    pub fn clear_element_import(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.element_import.len();

        // Update all opposite references
        for ref_id in &instance.element_import {
            if let Ok(target_js) = ElementImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ElementImport>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ElementImport::update(target_js);
                }
            }
        }
        }

        instance.element_import.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a PackageImport to package_import
    #[wasm_bindgen]
    pub fn add_package_import(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.package_import.contains(&ref_id) {
            return Ok(false);
        }

        instance.package_import.push(ref_id.clone());

        if let Ok(target_js) = PackageImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<PackageImport>(target_js) {
                target.importing_namespace = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = PackageImport::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a PackageImport from package_import
    #[wasm_bindgen]
    pub fn remove_package_import(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.package_import.iter().position(|x| x == &ref_id) {
            instance.package_import.remove(pos);

        if let Ok(target_js) = PackageImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<PackageImport>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = PackageImport::update(target_js);
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

    /// Clears all PackageImport from package_import
    #[wasm_bindgen]
    pub fn clear_package_import(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.package_import.len();

        // Update all opposite references
        for ref_id in &instance.package_import {
            if let Ok(target_js) = PackageImport::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<PackageImport>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = PackageImport::update(target_js);
                }
            }
        }
        }

        instance.package_import.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Constraint to owned_rule
    #[wasm_bindgen]
    pub fn add_owned_rule(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_rule.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_rule.push(ref_id.clone());

        if let Ok(target_js) = Constraint::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Constraint>(target_js) {
                target.context = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Constraint::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Constraint from owned_rule
    #[wasm_bindgen]
    pub fn remove_owned_rule(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_rule.iter().position(|x| x == &ref_id) {
            instance.owned_rule.remove(pos);

        if let Ok(target_js) = Constraint::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Constraint>(target_js) {
                target.context = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Constraint::update(target_js);
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

    /// Clears all Constraint from owned_rule
    #[wasm_bindgen]
    pub fn clear_owned_rule(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_rule.len();

        // Update all opposite references
        for ref_id in &instance.owned_rule {
            if let Ok(target_js) = Constraint::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Constraint>(target_js) {
                target.context = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Constraint::update(target_js);
                }
            }
        }
        }

        instance.owned_rule.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the owning_template_parameter reference
    #[wasm_bindgen]
    pub fn set_owning_template_parameter(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.owning_template_parameter {
            if let Ok(target_js) = TemplateParameter::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                    target.owned_parametered_element = None;
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = TemplateParameter::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = TemplateParameter::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                target.owned_parametered_element = Some(instance_id.clone());
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateParameter::update(target_js);
                }
            }
        }

        }

        instance.owning_template_parameter = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the template_parameter reference
    #[wasm_bindgen]
    pub fn set_template_parameter(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.template_parameter {
            if let Ok(target_js) = TemplateParameter::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = TemplateParameter::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = TemplateParameter::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateParameter>(target_js) {
                target.parametered_element = instance_id.clone();
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateParameter::update(target_js);
                }
            }
        }

        }

        instance.template_parameter = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a TemplateBinding to template_binding
    #[wasm_bindgen]
    pub fn add_template_binding(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.template_binding.contains(&ref_id) {
            return Ok(false);
        }

        instance.template_binding.push(ref_id.clone());

        if let Ok(target_js) = TemplateBinding::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateBinding>(target_js) {
                target.bound_element = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateBinding::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a TemplateBinding from template_binding
    #[wasm_bindgen]
    pub fn remove_template_binding(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.template_binding.iter().position(|x| x == &ref_id) {
            instance.template_binding.remove(pos);

        if let Ok(target_js) = TemplateBinding::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateBinding>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateBinding::update(target_js);
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

    /// Clears all TemplateBinding from template_binding
    #[wasm_bindgen]
    pub fn clear_template_binding(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.template_binding.len();

        // Update all opposite references
        for ref_id in &instance.template_binding {
            if let Ok(target_js) = TemplateBinding::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateBinding>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateBinding::update(target_js);
                }
            }
        }
        }

        instance.template_binding.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the owned_template_signature reference
    #[wasm_bindgen]
    pub fn set_owned_template_signature(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.owned_template_signature {
            if let Ok(target_js) = TemplateSignature::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateSignature>(target_js) {
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = TemplateSignature::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = TemplateSignature::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<TemplateSignature>(target_js) {
                target.template = instance_id.clone();
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = TemplateSignature::update(target_js);
                }
            }
        }

        }

        instance.owned_template_signature = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a Generalization to generalization
    #[wasm_bindgen]
    pub fn add_generalization(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.generalization.contains(&ref_id) {
            return Ok(false);
        }

        instance.generalization.push(ref_id.clone());

        if let Ok(target_js) = Generalization::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Generalization>(target_js) {
                target.specific = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Generalization::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Generalization from generalization
    #[wasm_bindgen]
    pub fn remove_generalization(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.generalization.iter().position(|x| x == &ref_id) {
            instance.generalization.remove(pos);

        if let Ok(target_js) = Generalization::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Generalization>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Generalization::update(target_js);
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

    /// Clears all Generalization from generalization
    #[wasm_bindgen]
    pub fn clear_generalization(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.generalization.len();

        // Update all opposite references
        for ref_id in &instance.generalization {
            if let Ok(target_js) = Generalization::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Generalization>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Generalization::update(target_js);
                }
            }
        }
        }

        instance.generalization.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a GeneralizationSet to powertype_extent
    #[wasm_bindgen]
    pub fn add_powertype_extent(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.powertype_extent.contains(&ref_id) {
            return Ok(false);
        }

        instance.powertype_extent.push(ref_id.clone());

        if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                target.powertype = Some(instance_id);
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

    /// Removes a GeneralizationSet from powertype_extent
    #[wasm_bindgen]
    pub fn remove_powertype_extent(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.powertype_extent.iter().position(|x| x == &ref_id) {
            instance.powertype_extent.remove(pos);

        if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                target.powertype = None;
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

    /// Clears all GeneralizationSet from powertype_extent
    #[wasm_bindgen]
    pub fn clear_powertype_extent(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.powertype_extent.len();

        // Update all opposite references
        for ref_id in &instance.powertype_extent {
            if let Ok(target_js) = GeneralizationSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<GeneralizationSet>(target_js) {
                target.powertype = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = GeneralizationSet::update(target_js);
                }
            }
        }
        }

        instance.powertype_extent.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Classifier to redefined_classifier
    #[wasm_bindgen]
    pub fn add_redefined_classifier(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.redefined_classifier.contains(&ref_id) {
            return Ok(false);
        }

        instance.redefined_classifier.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Classifier from redefined_classifier
    #[wasm_bindgen]
    pub fn remove_redefined_classifier(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.redefined_classifier.iter().position(|x| x == &ref_id) {
            instance.redefined_classifier.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Classifier from redefined_classifier
    #[wasm_bindgen]
    pub fn clear_redefined_classifier(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.redefined_classifier.len();

        instance.redefined_classifier.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Substitution to substitution
    #[wasm_bindgen]
    pub fn add_substitution(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.substitution.contains(&ref_id) {
            return Ok(false);
        }

        instance.substitution.push(ref_id.clone());

        if let Ok(target_js) = Substitution::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Substitution>(target_js) {
                target.substituting_classifier = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Substitution::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Substitution from substitution
    #[wasm_bindgen]
    pub fn remove_substitution(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.substitution.iter().position(|x| x == &ref_id) {
            instance.substitution.remove(pos);

        if let Ok(target_js) = Substitution::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Substitution>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Substitution::update(target_js);
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

    /// Clears all Substitution from substitution
    #[wasm_bindgen]
    pub fn clear_substitution(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.substitution.len();

        // Update all opposite references
        for ref_id in &instance.substitution {
            if let Ok(target_js) = Substitution::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Substitution>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Substitution::update(target_js);
                }
            }
        }
        }

        instance.substitution.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the representation reference
    #[wasm_bindgen]
    pub fn set_representation(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.representation = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a CollaborationUse to collaboration_use
    #[wasm_bindgen]
    pub fn add_collaboration_use(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.collaboration_use.contains(&ref_id) {
            return Ok(false);
        }

        instance.collaboration_use.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a CollaborationUse from collaboration_use
    #[wasm_bindgen]
    pub fn remove_collaboration_use(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.collaboration_use.iter().position(|x| x == &ref_id) {
            instance.collaboration_use.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all CollaborationUse from collaboration_use
    #[wasm_bindgen]
    pub fn clear_collaboration_use(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.collaboration_use.len();

        instance.collaboration_use.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a UseCase to owned_use_case
    #[wasm_bindgen]
    pub fn add_owned_use_case(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_use_case.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_use_case.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a UseCase from owned_use_case
    #[wasm_bindgen]
    pub fn remove_owned_use_case(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_use_case.iter().position(|x| x == &ref_id) {
            instance.owned_use_case.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all UseCase from owned_use_case
    #[wasm_bindgen]
    pub fn clear_owned_use_case(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_use_case.len();

        instance.owned_use_case.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a UseCase to use_case
    #[wasm_bindgen]
    pub fn add_use_case(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.use_case.contains(&ref_id) {
            return Ok(false);
        }

        instance.use_case.push(ref_id.clone());

        if let Ok(target_js) = UseCase::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<UseCase>(target_js) {
                if !target.subject.contains(&instance_id) {
                    target.subject.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = UseCase::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a UseCase from use_case
    #[wasm_bindgen]
    pub fn remove_use_case(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.use_case.iter().position(|x| x == &ref_id) {
            instance.use_case.remove(pos);

        if let Ok(target_js) = UseCase::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<UseCase>(target_js) {
                target.subject.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = UseCase::update(target_js);
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

    /// Clears all UseCase from use_case
    #[wasm_bindgen]
    pub fn clear_use_case(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.use_case.len();

        // Update all opposite references
        for ref_id in &instance.use_case {
            if let Ok(target_js) = UseCase::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<UseCase>(target_js) {
                target.subject.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = UseCase::update(target_js);
                }
            }
        }
        }

        instance.use_case.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Artifact to nested_artifact
    #[wasm_bindgen]
    pub fn add_nested_artifact(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.nested_artifact.contains(&ref_id) {
            return Ok(false);
        }

        instance.nested_artifact.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Artifact from nested_artifact
    #[wasm_bindgen]
    pub fn remove_nested_artifact(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.nested_artifact.iter().position(|x| x == &ref_id) {
            instance.nested_artifact.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Artifact from nested_artifact
    #[wasm_bindgen]
    pub fn clear_nested_artifact(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.nested_artifact.len();

        instance.nested_artifact.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Manifestation to manifestation
    #[wasm_bindgen]
    pub fn add_manifestation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.manifestation.contains(&ref_id) {
            return Ok(false);
        }

        instance.manifestation.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Manifestation from manifestation
    #[wasm_bindgen]
    pub fn remove_manifestation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.manifestation.iter().position(|x| x == &ref_id) {
            instance.manifestation.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Manifestation from manifestation
    #[wasm_bindgen]
    pub fn clear_manifestation(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.manifestation.len();

        instance.manifestation.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Operation to owned_operation
    #[wasm_bindgen]
    pub fn add_owned_operation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_operation.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_operation.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Operation from owned_operation
    #[wasm_bindgen]
    pub fn remove_owned_operation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_operation.iter().position(|x| x == &ref_id) {
            instance.owned_operation.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Operation from owned_operation
    #[wasm_bindgen]
    pub fn clear_owned_operation(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_operation.len();

        instance.owned_operation.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Property to owned_attribute
    #[wasm_bindgen]
    pub fn add_owned_attribute(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.owned_attribute.contains(&ref_id) {
            return Ok(false);
        }

        instance.owned_attribute.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Property from owned_attribute
    #[wasm_bindgen]
    pub fn remove_owned_attribute(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.owned_attribute.iter().position(|x| x == &ref_id) {
            instance.owned_attribute.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Property from owned_attribute
    #[wasm_bindgen]
    pub fn clear_owned_attribute(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_attribute.len();

        instance.owned_attribute.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the visibility field
    #[wasm_bindgen]
    pub fn set_visibility(instance_id: String, value: Option<VisibilityKind>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.visibility = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_leaf field
    #[wasm_bindgen]
    pub fn set_is_leaf(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_leaf = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_abstract field
    #[wasm_bindgen]
    pub fn set_is_abstract(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_abstract = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the file_name field
    #[wasm_bindgen]
    pub fn set_file_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Artifact = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.file_name = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl Artifact {
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

        // Validate all client_dependency references exist
        for id in &self.client_dependency {
            if !Dependency::exists(id.clone()) {
                errors.push(format!("Dependency {} not found", id));
            }
        }

        // Validate name_expression reference exists
        if let Some(id) = &self.name_expression {
            if !StringExpression::exists(id.clone()) {
                errors.push(format!("StringExpression {} not found", id));
            }
        }

        // Validate all element_import references exist
        for id in &self.element_import {
            if !ElementImport::exists(id.clone()) {
                errors.push(format!("ElementImport {} not found", id));
            }
        }

        // Validate all package_import references exist
        for id in &self.package_import {
            if !PackageImport::exists(id.clone()) {
                errors.push(format!("PackageImport {} not found", id));
            }
        }

        // Validate all owned_rule references exist
        for id in &self.owned_rule {
            if !Constraint::exists(id.clone()) {
                errors.push(format!("Constraint {} not found", id));
            }
        }

        // Validate owning_template_parameter reference exists
        if let Some(id) = &self.owning_template_parameter {
            if !TemplateParameter::exists(id.clone()) {
                errors.push(format!("TemplateParameter {} not found", id));
            }
        }

        // Validate template_parameter reference exists
        if let Some(id) = &self.template_parameter {
            if !TemplateParameter::exists(id.clone()) {
                errors.push(format!("TemplateParameter {} not found", id));
            }
        }

        // Validate all template_binding references exist
        for id in &self.template_binding {
            if !TemplateBinding::exists(id.clone()) {
                errors.push(format!("TemplateBinding {} not found", id));
            }
        }

        // Validate owned_template_signature reference exists
        if let Some(id) = &self.owned_template_signature {
            if !TemplateSignature::exists(id.clone()) {
                errors.push(format!("TemplateSignature {} not found", id));
            }
        }

        // Validate all generalization references exist
        for id in &self.generalization {
            if !Generalization::exists(id.clone()) {
                errors.push(format!("Generalization {} not found", id));
            }
        }

        // Validate all powertype_extent references exist
        for id in &self.powertype_extent {
            if !GeneralizationSet::exists(id.clone()) {
                errors.push(format!("GeneralizationSet {} not found", id));
            }
        }

        // Validate all redefined_classifier references exist
        for id in &self.redefined_classifier {
            if !Classifier::exists(id.clone()) {
                errors.push(format!("Classifier {} not found", id));
            }
        }

        // Validate all substitution references exist
        for id in &self.substitution {
            if !Substitution::exists(id.clone()) {
                errors.push(format!("Substitution {} not found", id));
            }
        }

        // Validate representation reference exists
        if let Some(id) = &self.representation {
            if !CollaborationUse::exists(id.clone()) {
                errors.push(format!("CollaborationUse {} not found", id));
            }
        }

        // Validate all collaboration_use references exist
        for id in &self.collaboration_use {
            if !CollaborationUse::exists(id.clone()) {
                errors.push(format!("CollaborationUse {} not found", id));
            }
        }

        // Validate all owned_use_case references exist
        for id in &self.owned_use_case {
            if !UseCase::exists(id.clone()) {
                errors.push(format!("UseCase {} not found", id));
            }
        }

        // Validate all use_case references exist
        for id in &self.use_case {
            if !UseCase::exists(id.clone()) {
                errors.push(format!("UseCase {} not found", id));
            }
        }

        // Validate all nested_artifact references exist
        for id in &self.nested_artifact {
            if !Artifact::exists(id.clone()) {
                errors.push(format!("Artifact {} not found", id));
            }
        }

        // Validate all manifestation references exist
        for id in &self.manifestation {
            if !Manifestation::exists(id.clone()) {
                errors.push(format!("Manifestation {} not found", id));
            }
        }

        // Validate all owned_operation references exist
        for id in &self.owned_operation {
            if !Operation::exists(id.clone()) {
                errors.push(format!("Operation {} not found", id));
            }
        }

        // Validate all owned_attribute references exist
        for id in &self.owned_attribute {
            if !Property::exists(id.clone()) {
                errors.push(format!("Property {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

