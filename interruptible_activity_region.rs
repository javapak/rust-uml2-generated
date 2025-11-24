// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           InterruptibleActivityRegion (struct)
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
use crate::activity::Activity;
use crate::activity_node::ActivityNode;
use crate::activity_edge::ActivityEdge;

lazy_static! {
    static ref INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY: Mutex<RefCell<HashMap<String, InterruptibleActivityRegion>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct InterruptibleActivityRegion {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub in_activity: Option<String>,
    pub node: Vec<String>,
    pub interrupting_edge: Vec<String>,
}

#[wasm_bindgen]
impl InterruptibleActivityRegion {
    /// Creates a new InterruptibleActivityRegion and returns its ID
    #[wasm_bindgen]
    pub fn create() -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            in_activity: None,
            node: Vec::new(),
            interrupting_edge: Vec::new(),
        };

        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a InterruptibleActivityRegion by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a InterruptibleActivityRegion instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a InterruptibleActivityRegion by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a InterruptibleActivityRegion exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all InterruptibleActivityRegion instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<InterruptibleActivityRegion> = INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of InterruptibleActivityRegion instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all InterruptibleActivityRegion instances
    #[wasm_bindgen]
    pub fn clear_all() {
        INTERRUPTIBLE_ACTIVITY_REGION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Adds a EAnnotation to e_annotations
    #[wasm_bindgen]
    pub fn add_e_annotation(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.owned_comment.len();

        instance.owned_comment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the in_activity reference
    #[wasm_bindgen]
    pub fn set_in_activity(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.in_activity {
            if let Ok(target_js) = Activity::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Activity>(target_js) {
                    target.group.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = Activity::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = Activity::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Activity>(target_js) {
                if !target.group.contains(&instance_id) {
                    target.group.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Activity::update(target_js);
                }
            }
        }

        }

        instance.in_activity = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a ActivityNode to node
    #[wasm_bindgen]
    pub fn add_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.node.contains(&ref_id) {
            return Ok(false);
        }

        instance.node.push(ref_id.clone());

        if let Ok(target_js) = ActivityNode::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityNode>(target_js) {
                if !target.in_interruptible_region.contains(&instance_id) {
                    target.in_interruptible_region.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityNode::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ActivityNode from node
    #[wasm_bindgen]
    pub fn remove_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.node.iter().position(|x| x == &ref_id) {
            instance.node.remove(pos);

        if let Ok(target_js) = ActivityNode::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityNode>(target_js) {
                target.in_interruptible_region.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityNode::update(target_js);
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

    /// Clears all ActivityNode from node
    #[wasm_bindgen]
    pub fn clear_node(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.node.len();

        // Update all opposite references
        for ref_id in &instance.node {
            if let Ok(target_js) = ActivityNode::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityNode>(target_js) {
                target.in_interruptible_region.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityNode::update(target_js);
                }
            }
        }
        }

        instance.node.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ActivityEdge to interrupting_edge
    #[wasm_bindgen]
    pub fn add_interrupting_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.interrupting_edge.contains(&ref_id) {
            return Ok(false);
        }

        instance.interrupting_edge.push(ref_id.clone());

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                target.interrupts = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityEdge::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ActivityEdge from interrupting_edge
    #[wasm_bindgen]
    pub fn remove_interrupting_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.interrupting_edge.iter().position(|x| x == &ref_id) {
            instance.interrupting_edge.remove(pos);

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                target.interrupts = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityEdge::update(target_js);
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

    /// Clears all ActivityEdge from interrupting_edge
    #[wasm_bindgen]
    pub fn clear_interrupting_edge(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.interrupting_edge.len();

        // Update all opposite references
        for ref_id in &instance.interrupting_edge {
            if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                target.interrupts = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityEdge::update(target_js);
                }
            }
        }
        }

        instance.interrupting_edge.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

}

impl InterruptibleActivityRegion {
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

        // Validate in_activity reference exists
        if let Some(id) = &self.in_activity {
            if !Activity::exists(id.clone()) {
                errors.push(format!("Activity {} not found", id));
            }
        }

        // Validate all node references exist
        for id in &self.node {
            if !ActivityNode::exists(id.clone()) {
                errors.push(format!("ActivityNode {} not found", id));
            }
        }

        // Validate all interrupting_edge references exist
        for id in &self.interrupting_edge {
            if !ActivityEdge::exists(id.clone()) {
                errors.push(format!("ActivityEdge {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

