// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           FinalState (struct)
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
use crate::region::Region;
use crate::state_machine::StateMachine;
use crate::connection_point_reference::ConnectionPointReference;
use crate::pseudostate::Pseudostate;
use crate::state::State;
use crate::behavior::Behavior;
use crate::trigger::Trigger;

lazy_static! {
    static ref FINAL_STATE_REGISTRY: Mutex<RefCell<HashMap<String, FinalState>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct FinalState {
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
    pub container: Option<String>,
    pub submachine: Option<String>,
    pub connection: Vec<String>,
    pub connection_point: Vec<String>,
    pub redefined_state: Option<String>,
    pub state_invariant: Option<String>,
    pub entry: Option<String>,
    pub exit: Option<String>,
    pub do_activity: Option<String>,
    pub deferrable_trigger: Vec<String>,
    pub region: Vec<String>,
}

#[wasm_bindgen]
impl FinalState {
    /// Creates a new FinalState and returns its ID
    #[wasm_bindgen]
    pub fn create(is_leaf: String) -> String {
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
            container: None,
            submachine: None,
            connection: Vec::new(),
            connection_point: Vec::new(),
            redefined_state: None,
            state_invariant: None,
            entry: None,
            exit: None,
            do_activity: None,
            deferrable_trigger: Vec::new(),
            region: Vec::new(),
        };

        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a FinalState by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a FinalState instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: FinalState = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a FinalState by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a FinalState exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all FinalState instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<FinalState> = FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of FinalState instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all FinalState instances
    #[wasm_bindgen]
    pub fn clear_all() {
        FINAL_STATE_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds FinalState instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<FinalState> = FINAL_STATE_REGISTRY.lock().unwrap()
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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

    /// Sets the container reference
    #[wasm_bindgen]
    pub fn set_container(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.container {
            if let Ok(target_js) = Region::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Region>(target_js) {
                    target.subvertex.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = Region::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = Region::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Region>(target_js) {
                if !target.subvertex.contains(&instance_id) {
                    target.subvertex.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Region::update(target_js);
                }
            }
        }

        }

        instance.container = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the submachine reference
    #[wasm_bindgen]
    pub fn set_submachine(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.submachine {
            if let Ok(target_js) = StateMachine::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<StateMachine>(target_js) {
                    target.submachine_state.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = StateMachine::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = StateMachine::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<StateMachine>(target_js) {
                if !target.submachine_state.contains(&instance_id) {
                    target.submachine_state.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = StateMachine::update(target_js);
                }
            }
        }

        }

        instance.submachine = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a ConnectionPointReference to connection
    #[wasm_bindgen]
    pub fn add_connection(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.connection.contains(&ref_id) {
            return Ok(false);
        }

        instance.connection.push(ref_id.clone());

        if let Ok(target_js) = ConnectionPointReference::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ConnectionPointReference>(target_js) {
                target.state = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ConnectionPointReference::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ConnectionPointReference from connection
    #[wasm_bindgen]
    pub fn remove_connection(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.connection.iter().position(|x| x == &ref_id) {
            instance.connection.remove(pos);

        if let Ok(target_js) = ConnectionPointReference::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ConnectionPointReference>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ConnectionPointReference::update(target_js);
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

    /// Clears all ConnectionPointReference from connection
    #[wasm_bindgen]
    pub fn clear_connection(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.connection.len();

        // Update all opposite references
        for ref_id in &instance.connection {
            if let Ok(target_js) = ConnectionPointReference::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ConnectionPointReference>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ConnectionPointReference::update(target_js);
                }
            }
        }
        }

        instance.connection.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Pseudostate to connection_point
    #[wasm_bindgen]
    pub fn add_connection_point(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.connection_point.contains(&ref_id) {
            return Ok(false);
        }

        instance.connection_point.push(ref_id.clone());

        if let Ok(target_js) = Pseudostate::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Pseudostate>(target_js) {
                target.state = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Pseudostate::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Pseudostate from connection_point
    #[wasm_bindgen]
    pub fn remove_connection_point(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.connection_point.iter().position(|x| x == &ref_id) {
            instance.connection_point.remove(pos);

        if let Ok(target_js) = Pseudostate::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Pseudostate>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Pseudostate::update(target_js);
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

    /// Clears all Pseudostate from connection_point
    #[wasm_bindgen]
    pub fn clear_connection_point(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.connection_point.len();

        // Update all opposite references
        for ref_id in &instance.connection_point {
            if let Ok(target_js) = Pseudostate::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Pseudostate>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Pseudostate::update(target_js);
                }
            }
        }
        }

        instance.connection_point.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the redefined_state reference
    #[wasm_bindgen]
    pub fn set_redefined_state(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.redefined_state = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the state_invariant reference
    #[wasm_bindgen]
    pub fn set_state_invariant(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.state_invariant = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the entry reference
    #[wasm_bindgen]
    pub fn set_entry(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.entry = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the exit reference
    #[wasm_bindgen]
    pub fn set_exit(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.exit = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the do_activity reference
    #[wasm_bindgen]
    pub fn set_do_activity(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.do_activity = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a Trigger to deferrable_trigger
    #[wasm_bindgen]
    pub fn add_deferrable_trigger(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.deferrable_trigger.contains(&ref_id) {
            return Ok(false);
        }

        instance.deferrable_trigger.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Trigger from deferrable_trigger
    #[wasm_bindgen]
    pub fn remove_deferrable_trigger(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.deferrable_trigger.iter().position(|x| x == &ref_id) {
            instance.deferrable_trigger.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Trigger from deferrable_trigger
    #[wasm_bindgen]
    pub fn clear_deferrable_trigger(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.deferrable_trigger.len();

        instance.deferrable_trigger.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Region to region
    #[wasm_bindgen]
    pub fn add_region(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.region.contains(&ref_id) {
            return Ok(false);
        }

        instance.region.push(ref_id.clone());

        if let Ok(target_js) = Region::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Region>(target_js) {
                target.state = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Region::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Region from region
    #[wasm_bindgen]
    pub fn remove_region(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.region.iter().position(|x| x == &ref_id) {
            instance.region.remove(pos);

        if let Ok(target_js) = Region::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Region>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Region::update(target_js);
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

    /// Clears all Region from region
    #[wasm_bindgen]
    pub fn clear_region(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.region.len();

        // Update all opposite references
        for ref_id in &instance.region {
            if let Ok(target_js) = Region::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Region>(target_js) {
                target.state = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Region::update(target_js);
                }
            }
        }
        }

        instance.region.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: FinalState = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_leaf = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl FinalState {
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

        // Validate container reference exists
        if let Some(id) = &self.container {
            if !Region::exists(id.clone()) {
                errors.push(format!("Region {} not found", id));
            }
        }

        // Validate submachine reference exists
        if let Some(id) = &self.submachine {
            if !StateMachine::exists(id.clone()) {
                errors.push(format!("StateMachine {} not found", id));
            }
        }

        // Validate all connection references exist
        for id in &self.connection {
            if !ConnectionPointReference::exists(id.clone()) {
                errors.push(format!("ConnectionPointReference {} not found", id));
            }
        }

        // Validate all connection_point references exist
        for id in &self.connection_point {
            if !Pseudostate::exists(id.clone()) {
                errors.push(format!("Pseudostate {} not found", id));
            }
        }

        // Validate redefined_state reference exists
        if let Some(id) = &self.redefined_state {
            if !State::exists(id.clone()) {
                errors.push(format!("State {} not found", id));
            }
        }

        // Validate state_invariant reference exists
        if let Some(id) = &self.state_invariant {
            if !Constraint::exists(id.clone()) {
                errors.push(format!("Constraint {} not found", id));
            }
        }

        // Validate entry reference exists
        if let Some(id) = &self.entry {
            if !Behavior::exists(id.clone()) {
                errors.push(format!("Behavior {} not found", id));
            }
        }

        // Validate exit reference exists
        if let Some(id) = &self.exit {
            if !Behavior::exists(id.clone()) {
                errors.push(format!("Behavior {} not found", id));
            }
        }

        // Validate do_activity reference exists
        if let Some(id) = &self.do_activity {
            if !Behavior::exists(id.clone()) {
                errors.push(format!("Behavior {} not found", id));
            }
        }

        // Validate all deferrable_trigger references exist
        for id in &self.deferrable_trigger {
            if !Trigger::exists(id.clone()) {
                errors.push(format!("Trigger {} not found", id));
            }
        }

        // Validate all region references exist
        for id in &self.region {
            if !Region::exists(id.clone()) {
                errors.push(format!("Region {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

