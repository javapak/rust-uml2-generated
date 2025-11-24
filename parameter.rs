// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Parameter (struct)
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
use crate::type_::Type;
use crate::template_parameter::TemplateParameter;
use crate::value_specification::ValueSpecification;
use crate::parameter_set::ParameterSet;
use crate::parameter_direction_kind::ParameterDirectionKind;
use crate::parameter_effect_kind::ParameterEffectKind;

lazy_static! {
    static ref PARAMETER_REGISTRY: Mutex<RefCell<HashMap<String, Parameter>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct Parameter {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub name: Option<String>,
    pub visibility: Option<VisibilityKind>,
    pub client_dependency: Vec<String>,
    pub name_expression: Option<String>,
    pub type_: Option<String>,
    pub owning_template_parameter: Option<String>,
    pub template_parameter: Option<String>,
    pub is_ordered: String,
    pub is_unique: String,
    pub upper_value: Option<String>,
    pub lower_value: Option<String>,
    pub parameter_set: Vec<String>,
    pub direction: ParameterDirectionKind,
    pub default_value: Option<String>,
    pub is_exception: String,
    pub is_stream: String,
    pub effect: Option<ParameterEffectKind>,
}

#[wasm_bindgen]
impl Parameter {
    /// Creates a new Parameter and returns its ID
    #[wasm_bindgen]
    pub fn create(is_ordered: String, is_unique: String, direction: ParameterDirectionKind, is_exception: String, is_stream: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            type_: None,
            owning_template_parameter: None,
            template_parameter: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            parameter_set: Vec::new(),
            direction: direction,
            default_value: None,
            is_exception: is_exception,
            is_stream: is_stream,
            effect: None,
        };

        PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a Parameter by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a Parameter instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: Parameter = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a Parameter by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a Parameter exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all Parameter instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<Parameter> = PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of Parameter instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        PARAMETER_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all Parameter instances
    #[wasm_bindgen]
    pub fn clear_all() {
        PARAMETER_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds Parameter instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<Parameter> = PARAMETER_REGISTRY.lock().unwrap()
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.name_expression = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the type_ reference
    #[wasm_bindgen]
    pub fn set_type_(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.type_ = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the owning_template_parameter reference
    #[wasm_bindgen]
    pub fn set_owning_template_parameter(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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

    /// Sets the upper_value reference
    #[wasm_bindgen]
    pub fn set_upper_value(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.upper_value = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the lower_value reference
    #[wasm_bindgen]
    pub fn set_lower_value(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.lower_value = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a ParameterSet to parameter_set
    #[wasm_bindgen]
    pub fn add_parameter_set(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.parameter_set.contains(&ref_id) {
            return Ok(false);
        }

        instance.parameter_set.push(ref_id.clone());

        if let Ok(target_js) = ParameterSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ParameterSet>(target_js) {
                if !target.parameter.contains(&instance_id) {
                    target.parameter.push(instance_id);
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ParameterSet::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a ParameterSet from parameter_set
    #[wasm_bindgen]
    pub fn remove_parameter_set(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.parameter_set.iter().position(|x| x == &ref_id) {
            instance.parameter_set.remove(pos);

        if let Ok(target_js) = ParameterSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ParameterSet>(target_js) {
                target.parameter.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ParameterSet::update(target_js);
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

    /// Clears all ParameterSet from parameter_set
    #[wasm_bindgen]
    pub fn clear_parameter_set(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.parameter_set.len();

        // Update all opposite references
        for ref_id in &instance.parameter_set {
            if let Ok(target_js) = ParameterSet::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<ParameterSet>(target_js) {
                target.parameter.retain(|x| x != &instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = ParameterSet::update(target_js);
                }
            }
        }
        }

        instance.parameter_set.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the default_value reference
    #[wasm_bindgen]
    pub fn set_default_value(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.default_value = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.visibility = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_ordered field
    #[wasm_bindgen]
    pub fn set_is_ordered(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_ordered = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_unique field
    #[wasm_bindgen]
    pub fn set_is_unique(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_unique = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the direction field
    #[wasm_bindgen]
    pub fn set_direction(instance_id: String, value: ParameterDirectionKind) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.direction = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_exception field
    #[wasm_bindgen]
    pub fn set_is_exception(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_exception = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_stream field
    #[wasm_bindgen]
    pub fn set_is_stream(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_stream = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the effect field
    #[wasm_bindgen]
    pub fn set_effect(instance_id: String, value: Option<ParameterEffectKind>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: Parameter = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.effect = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl Parameter {
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

        // Validate type_ reference exists
        if let Some(id) = &self.type_ {
            if !Type::exists(id.clone()) {
                errors.push(format!("Type {} not found", id));
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

        // Validate upper_value reference exists
        if let Some(id) = &self.upper_value {
            if !ValueSpecification::exists(id.clone()) {
                errors.push(format!("ValueSpecification {} not found", id));
            }
        }

        // Validate lower_value reference exists
        if let Some(id) = &self.lower_value {
            if !ValueSpecification::exists(id.clone()) {
                errors.push(format!("ValueSpecification {} not found", id));
            }
        }

        // Validate all parameter_set references exist
        for id in &self.parameter_set {
            if !ParameterSet::exists(id.clone()) {
                errors.push(format!("ParameterSet {} not found", id));
            }
        }

        // Validate default_value reference exists
        if let Some(id) = &self.default_value {
            if !ValueSpecification::exists(id.clone()) {
                errors.push(format!("ValueSpecification {} not found", id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

