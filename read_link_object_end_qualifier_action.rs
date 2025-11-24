// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ReadLinkObjectEndQualifierAction (struct)
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
use crate::structured_activity_node::StructuredActivityNode;
use crate::activity::Activity;
use crate::activity_edge::ActivityEdge;
use crate::activity_partition::ActivityPartition;
use crate::interruptible_activity_region::InterruptibleActivityRegion;
use crate::activity_node::ActivityNode;
use crate::exception_handler::ExceptionHandler;
use crate::constraint::Constraint;
use crate::input_pin::InputPin;
use crate::output_pin::OutputPin;
use crate::property::Property;

lazy_static! {
    static ref READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY: Mutex<RefCell<HashMap<String, ReadLinkObjectEndQualifierAction>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct ReadLinkObjectEndQualifierAction {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub name: Option<String>,
    pub visibility: Option<VisibilityKind>,
    pub client_dependency: Vec<String>,
    pub name_expression: Option<String>,
    pub is_leaf: String,
    pub in_structured_node: Option<String>,
    pub activity: Option<String>,
    pub outgoing: Vec<String>,
    pub incoming: Vec<String>,
    pub in_partition: Vec<String>,
    pub in_interruptible_region: Vec<String>,
    pub redefined_node: Vec<String>,
    pub handler: Vec<String>,
    pub local_precondition: Vec<String>,
    pub local_postcondition: Vec<String>,
    pub object: String,
    pub result: String,
    pub qualifier: String,
}

#[wasm_bindgen]
impl ReadLinkObjectEndQualifierAction {
    /// Creates a new ReadLinkObjectEndQualifierAction and returns its ID
    #[wasm_bindgen]
    pub fn create(is_leaf: String, object: String, result: String, qualifier: String) -> String {
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
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            handler: Vec::new(),
            local_precondition: Vec::new(),
            local_postcondition: Vec::new(),
            object: object,
            result: result,
            qualifier: qualifier,
        };

        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a ReadLinkObjectEndQualifierAction by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a ReadLinkObjectEndQualifierAction instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a ReadLinkObjectEndQualifierAction by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a ReadLinkObjectEndQualifierAction exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all ReadLinkObjectEndQualifierAction instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<ReadLinkObjectEndQualifierAction> = READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of ReadLinkObjectEndQualifierAction instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all ReadLinkObjectEndQualifierAction instances
    #[wasm_bindgen]
    pub fn clear_all() {
        READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds ReadLinkObjectEndQualifierAction instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<ReadLinkObjectEndQualifierAction> = READ_LINK_OBJECT_END_QUALIFIER_ACTION_REGISTRY.lock().unwrap()
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name_expression = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the in_structured_node reference
    #[wasm_bindgen]
    pub fn set_in_structured_node(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.in_structured_node {
            if let Ok(target_js) = StructuredActivityNode::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<StructuredActivityNode>(target_js) {
                    target.node.retain(|x| x != &instance_id);
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
                if !target.node.contains(&instance_id) {
                    target.node.push(instance_id.clone());
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.activity {
            if let Ok(target_js) = Activity::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Activity>(target_js) {
                    target.node.retain(|x| x != &instance_id);
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
                if !target.node.contains(&instance_id) {
                    target.node.push(instance_id.clone());
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

    /// Adds a ActivityEdge to outgoing
    #[wasm_bindgen]
    pub fn add_outgoing(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.outgoing.contains(&ref_id) {
            return Ok(false);
        }

        instance.outgoing.push(ref_id.clone());

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                target.source = instance_id;
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

    /// Removes a ActivityEdge from outgoing
    #[wasm_bindgen]
    pub fn remove_outgoing(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.outgoing.iter().position(|x| x == &ref_id) {
            instance.outgoing.remove(pos);

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
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

    /// Clears all ActivityEdge from outgoing
    #[wasm_bindgen]
    pub fn clear_outgoing(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.outgoing.len();

        // Update all opposite references
        for ref_id in &instance.outgoing {
            if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityEdge::update(target_js);
                }
            }
        }
        }

        instance.outgoing.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ActivityEdge to incoming
    #[wasm_bindgen]
    pub fn add_incoming(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.incoming.contains(&ref_id) {
            return Ok(false);
        }

        instance.incoming.push(ref_id.clone());

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                target.target = instance_id;
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

    /// Removes a ActivityEdge from incoming
    #[wasm_bindgen]
    pub fn remove_incoming(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.incoming.iter().position(|x| x == &ref_id) {
            instance.incoming.remove(pos);

        if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
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

    /// Clears all ActivityEdge from incoming
    #[wasm_bindgen]
    pub fn clear_incoming(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.incoming.len();

        // Update all opposite references
        for ref_id in &instance.incoming {
            if let Ok(target_js) = ActivityEdge::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityEdge>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ActivityEdge::update(target_js);
                }
            }
        }
        }

        instance.incoming.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ActivityPartition to in_partition
    #[wasm_bindgen]
    pub fn add_in_partition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.in_partition.contains(&ref_id) {
            return Ok(false);
        }

        instance.in_partition.push(ref_id.clone());

        if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                if !target.node.contains(&instance_id) {
                    target.node.push(instance_id);
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.in_partition.iter().position(|x| x == &ref_id) {
            instance.in_partition.remove(pos);

        if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                target.node.retain(|x| x != &instance_id);
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.in_partition.len();

        // Update all opposite references
        for ref_id in &instance.in_partition {
            if let Ok(target_js) = ActivityPartition::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ActivityPartition>(target_js) {
                target.node.retain(|x| x != &instance_id);
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

    /// Adds a InterruptibleActivityRegion to in_interruptible_region
    #[wasm_bindgen]
    pub fn add_in_interruptible_region(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.in_interruptible_region.contains(&ref_id) {
            return Ok(false);
        }

        instance.in_interruptible_region.push(ref_id.clone());

        if let Ok(target_js) = InterruptibleActivityRegion::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<InterruptibleActivityRegion>(target_js) {
                if !target.node.contains(&instance_id) {
                    target.node.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = InterruptibleActivityRegion::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a InterruptibleActivityRegion from in_interruptible_region
    #[wasm_bindgen]
    pub fn remove_in_interruptible_region(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.in_interruptible_region.iter().position(|x| x == &ref_id) {
            instance.in_interruptible_region.remove(pos);

        if let Ok(target_js) = InterruptibleActivityRegion::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<InterruptibleActivityRegion>(target_js) {
                target.node.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = InterruptibleActivityRegion::update(target_js);
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

    /// Clears all InterruptibleActivityRegion from in_interruptible_region
    #[wasm_bindgen]
    pub fn clear_in_interruptible_region(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.in_interruptible_region.len();

        // Update all opposite references
        for ref_id in &instance.in_interruptible_region {
            if let Ok(target_js) = InterruptibleActivityRegion::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<InterruptibleActivityRegion>(target_js) {
                target.node.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = InterruptibleActivityRegion::update(target_js);
                }
            }
        }
        }

        instance.in_interruptible_region.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ActivityNode to redefined_node
    #[wasm_bindgen]
    pub fn add_redefined_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.redefined_node.contains(&ref_id) {
            return Ok(false);
        }

        instance.redefined_node.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ActivityNode from redefined_node
    #[wasm_bindgen]
    pub fn remove_redefined_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.redefined_node.iter().position(|x| x == &ref_id) {
            instance.redefined_node.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all ActivityNode from redefined_node
    #[wasm_bindgen]
    pub fn clear_redefined_node(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.redefined_node.len();

        instance.redefined_node.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a ExceptionHandler to handler
    #[wasm_bindgen]
    pub fn add_handler(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.handler.contains(&ref_id) {
            return Ok(false);
        }

        instance.handler.push(ref_id.clone());

        if let Ok(target_js) = ExceptionHandler::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ExceptionHandler>(target_js) {
                target.protected_node = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ExceptionHandler::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ExceptionHandler from handler
    #[wasm_bindgen]
    pub fn remove_handler(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.handler.iter().position(|x| x == &ref_id) {
            instance.handler.remove(pos);

        if let Ok(target_js) = ExceptionHandler::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ExceptionHandler>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ExceptionHandler::update(target_js);
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

    /// Clears all ExceptionHandler from handler
    #[wasm_bindgen]
    pub fn clear_handler(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.handler.len();

        // Update all opposite references
        for ref_id in &instance.handler {
            if let Ok(target_js) = ExceptionHandler::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ExceptionHandler>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ExceptionHandler::update(target_js);
                }
            }
        }
        }

        instance.handler.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Constraint to local_precondition
    #[wasm_bindgen]
    pub fn add_local_precondition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.local_precondition.contains(&ref_id) {
            return Ok(false);
        }

        instance.local_precondition.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Constraint from local_precondition
    #[wasm_bindgen]
    pub fn remove_local_precondition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.local_precondition.iter().position(|x| x == &ref_id) {
            instance.local_precondition.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Constraint from local_precondition
    #[wasm_bindgen]
    pub fn clear_local_precondition(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.local_precondition.len();

        instance.local_precondition.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a Constraint to local_postcondition
    #[wasm_bindgen]
    pub fn add_local_postcondition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.local_postcondition.contains(&ref_id) {
            return Ok(false);
        }

        instance.local_postcondition.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Constraint from local_postcondition
    #[wasm_bindgen]
    pub fn remove_local_postcondition(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.local_postcondition.iter().position(|x| x == &ref_id) {
            instance.local_postcondition.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Constraint from local_postcondition
    #[wasm_bindgen]
    pub fn clear_local_postcondition(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.local_postcondition.len();

        instance.local_postcondition.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the object reference
    #[wasm_bindgen]
    pub fn set_object(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.object = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the result reference
    #[wasm_bindgen]
    pub fn set_result(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.result = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the qualifier reference
    #[wasm_bindgen]
    pub fn set_qualifier(instance_id: String, ref_id: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.qualifier = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ReadLinkObjectEndQualifierAction = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_leaf = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl ReadLinkObjectEndQualifierAction {
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

        // Validate all outgoing references exist
        for id in &self.outgoing {
            if !ActivityEdge::exists(id.clone()) {
                errors.push(format!("ActivityEdge {} not found", id));
            }
        }

        // Validate all incoming references exist
        for id in &self.incoming {
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

        // Validate all in_interruptible_region references exist
        for id in &self.in_interruptible_region {
            if !InterruptibleActivityRegion::exists(id.clone()) {
                errors.push(format!("InterruptibleActivityRegion {} not found", id));
            }
        }

        // Validate all redefined_node references exist
        for id in &self.redefined_node {
            if !ActivityNode::exists(id.clone()) {
                errors.push(format!("ActivityNode {} not found", id));
            }
        }

        // Validate all handler references exist
        for id in &self.handler {
            if !ExceptionHandler::exists(id.clone()) {
                errors.push(format!("ExceptionHandler {} not found", id));
            }
        }

        // Validate all local_precondition references exist
        for id in &self.local_precondition {
            if !Constraint::exists(id.clone()) {
                errors.push(format!("Constraint {} not found", id));
            }
        }

        // Validate all local_postcondition references exist
        for id in &self.local_postcondition {
            if !Constraint::exists(id.clone()) {
                errors.push(format!("Constraint {} not found", id));
            }
        }

        // Validate object reference exists
        if !InputPin::exists(self.object.clone()) {
            errors.push(format!("InputPin {} not found", self.object));
        }

        // Validate result reference exists
        if !OutputPin::exists(self.result.clone()) {
            errors.push(format!("OutputPin {} not found", self.result));
        }

        // Validate qualifier reference exists
        if !Property::exists(self.qualifier.clone()) {
            errors.push(format!("Property {} not found", self.qualifier));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

