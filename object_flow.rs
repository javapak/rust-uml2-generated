// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ObjectFlow (struct)
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
use crate::activity_node::ActivityNode;
use crate::activity_edge::ActivityEdge;
use crate::activity_partition::ActivityPartition;
use crate::value_specification::ValueSpecification;
use crate::interruptible_activity_region::InterruptibleActivityRegion;
use crate::structured_activity_node::StructuredActivityNode;
use crate::activity::Activity;
use crate::behavior::Behavior;

lazy_static! {
    static ref OBJECT_FLOW_REGISTRY: Mutex<RefCell<HashMap<String, ObjectFlow>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct ObjectFlow {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub name: Option<String>,
    pub visibility: Option<VisibilityKind>,
    pub client_dependency: Vec<String>,
    pub name_expression: Option<String>,
    pub is_leaf: String,
    pub source: String,
    pub target: String,
    pub redefined_edge: Vec<String>,
    pub in_partition: Vec<String>,
    pub guard: String,
    pub weight: String,
    pub interrupts: Option<String>,
    pub in_structured_node: Option<String>,
    pub activity: Option<String>,
    pub is_multicast: String,
    pub is_multireceive: String,
    pub transformation: Option<String>,
    pub selection: Option<String>,
}

#[wasm_bindgen]
impl ObjectFlow {
    /// Creates a new ObjectFlow and returns its ID
    #[wasm_bindgen]
    pub fn create(is_leaf: String, source: String, target: String, guard: String, weight: String, is_multicast: String, is_multireceive: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf: is_leaf,
            source: source,
            target: target,
            redefined_edge: Vec::new(),
            in_partition: Vec::new(),
            guard: guard,
            weight: weight,
            interrupts: None,
            in_structured_node: None,
            activity: None,
            is_multicast: is_multicast,
            is_multireceive: is_multireceive,
            transformation: None,
            selection: None,
        };

        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a ObjectFlow by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a ObjectFlow instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: ObjectFlow = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a ObjectFlow by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a ObjectFlow exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all ObjectFlow instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<ObjectFlow> = OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of ObjectFlow instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all ObjectFlow instances
    #[wasm_bindgen]
    pub fn clear_all() {
        OBJECT_FLOW_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds ObjectFlow instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<ObjectFlow> = OBJECT_FLOW_REGISTRY.lock().unwrap()
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name_expression = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the source reference
    #[wasm_bindgen]
    pub fn set_source(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Ok(target_js) = ActivityNode::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityNode>(target_js) {
                if !target.outgoing.contains(&instance_id) {
                    target.outgoing.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityNode::update(target_js);
                }
            }
        }

        instance.source = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the target reference
    #[wasm_bindgen]
    pub fn set_target(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Ok(target_js) = ActivityNode::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityNode>(target_js) {
                if !target.incoming.contains(&instance_id) {
                    target.incoming.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityNode::update(target_js);
                }
            }
        }

        instance.target = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a ActivityEdge to redefined_edge
    #[wasm_bindgen]
    pub fn add_redefined_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.redefined_edge.contains(&ref_id) {
            return Ok(false);
        }

        instance.redefined_edge.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ActivityEdge from redefined_edge
    #[wasm_bindgen]
    pub fn remove_redefined_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.redefined_edge.iter().position(|x| x == &ref_id) {
            instance.redefined_edge.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all ActivityEdge from redefined_edge
    #[wasm_bindgen]
    pub fn clear_redefined_edge(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.redefined_edge.len();

        instance.redefined_edge.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ActivityPartition to in_partition
    #[wasm_bindgen]
    pub fn add_in_partition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.in_partition.contains(&ref_id) {
            return Ok(false);
        }

        instance.in_partition.push(ref_id.clone());

        if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                if !target.edge.contains(&instance_id) {
                    target.edge.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityPartition::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ActivityPartition from in_partition
    #[wasm_bindgen]
    pub fn remove_in_partition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.in_partition.iter().position(|x| x == &ref_id) {
            instance.in_partition.remove(pos);

        if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                target.edge.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityPartition::update(target_js);
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

    /// Clears all ActivityPartition from in_partition
    #[wasm_bindgen]
    pub fn clear_in_partition(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.in_partition.len();

        // Update all opposite references
        for ref_id in &instance.in_partition {
            if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                target.edge.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityPartition::update(target_js);
                }
            }
        }
        }

        instance.in_partition.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the guard reference
    #[wasm_bindgen]
    pub fn set_guard(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.guard = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the weight reference
    #[wasm_bindgen]
    pub fn set_weight(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.weight = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the interrupts reference
    #[wasm_bindgen]
    pub fn set_interrupts(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.interrupts {
            if let Ok(target_js) = InterruptibleActivityRegion::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<InterruptibleActivityRegion>(target_js) {
                    target.interrupting_edge.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = InterruptibleActivityRegion::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = InterruptibleActivityRegion::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<InterruptibleActivityRegion>(target_js) {
                if !target.interrupting_edge.contains(&instance_id) {
                    target.interrupting_edge.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = InterruptibleActivityRegion::update(target_js);
                }
            }
        }

        }

        instance.interrupts = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the in_structured_node reference
    #[wasm_bindgen]
    pub fn set_in_structured_node(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.in_structured_node {
            if let Ok(target_js) = StructuredActivityNode::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<StructuredActivityNode>(target_js) {
                    target.edge.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = StructuredActivityNode::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = StructuredActivityNode::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<StructuredActivityNode>(target_js) {
                if !target.edge.contains(&instance_id) {
                    target.edge.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = StructuredActivityNode::update(target_js);
                }
            }
        }

        }

        instance.in_structured_node = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the activity reference
    #[wasm_bindgen]
    pub fn set_activity(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.activity {
            if let Ok(target_js) = Activity::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Activity>(target_js) {
                    target.edge.retain(|x| x != &instance_id);
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
                if !target.edge.contains(&instance_id) {
                    target.edge.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Activity::update(target_js);
                }
            }
        }

        }

        instance.activity = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the transformation reference
    #[wasm_bindgen]
    pub fn set_transformation(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.transformation = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the selection reference
    #[wasm_bindgen]
    pub fn set_selection(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.selection = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_leaf = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_multicast field
    #[wasm_bindgen]
    pub fn set_is_multicast(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_multicast = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_multireceive field
    #[wasm_bindgen]
    pub fn set_is_multireceive(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ObjectFlow = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_multireceive = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl ObjectFlow {
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

        // Validate source reference exists
        if !ActivityNode::exists(self.source.clone()) {
            errors.push(format!("ActivityNode {} not found", self.source));
        }

        // Validate target reference exists
        if !ActivityNode::exists(self.target.clone()) {
            errors.push(format!("ActivityNode {} not found", self.target));
        }

        // Validate all redefined_edge references exist
        for id in &self.redefined_edge {
            if !ActivityEdge::exists(id.clone()) {
                errors.push(format!("ActivityEdge {} not found", id));
            }
        }

        // Validate all in_partition references exist
        for id in &self.in_partition {
            if !ActivityPartition::exists(id.clone()) {
                errors.push(format!("ActivityPartition {} not found", id));
            }
        }

        // Validate guard reference exists
        if !ValueSpecification::exists(self.guard.clone()) {
            errors.push(format!("ValueSpecification {} not found", self.guard));
        }

        // Validate weight reference exists
        if !ValueSpecification::exists(self.weight.clone()) {
            errors.push(format!("ValueSpecification {} not found", self.weight));
        }

        // Validate interrupts reference exists
        if let Some(id) = &self.interrupts {
            if !InterruptibleActivityRegion::exists(id.clone()) {
                errors.push(format!("InterruptibleActivityRegion {} not found", id));
            }
        }

        // Validate in_structured_node reference exists
        if let Some(id) = &self.in_structured_node {
            if !StructuredActivityNode::exists(id.clone()) {
                errors.push(format!("StructuredActivityNode {} not found", id));
            }
        }

        // Validate activity reference exists
        if let Some(id) = &self.activity {
            if !Activity::exists(id.clone()) {
                errors.push(format!("Activity {} not found", id));
            }
        }

        // Validate transformation reference exists
        if let Some(id) = &self.transformation {
            if !Behavior::exists(id.clone()) {
                errors.push(format!("Behavior {} not found", id));
            }
        }

        // Validate selection reference exists
        if let Some(id) = &self.selection {
            if !Behavior::exists(id.clone()) {
                errors.push(format!("Behavior {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

