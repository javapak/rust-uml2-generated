// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ExtensionEnd (struct)
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
use crate::value_specification::ValueSpecification;
use crate::template_parameter::TemplateParameter;
use crate::deployment::Deployment;
use crate::template_binding::TemplateBinding;
use crate::template_signature::TemplateSignature;
use crate::data_type::DataType;
use crate::aggregation_kind::AggregationKind;
use crate::property::Property;
use crate::association::Association;

lazy_static! {
    static ref EXTENSION_END_REGISTRY: Mutex<RefCell<HashMap<String, ExtensionEnd>>> = 
        Mutex::new(RefCell::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct ExtensionEnd {
    /// Unique identifier for this instance
    pub id: String,
    pub e_annotations: Vec<String>,
    pub owned_comment: Vec<String>,
    pub name: Option<String>,
    pub visibility: Option<VisibilityKind>,
    pub client_dependency: Vec<String>,
    pub name_expression: Option<String>,
    pub is_leaf: String,
    pub is_static: String,
    pub type_: Option<String>,
    pub is_ordered: String,
    pub is_unique: String,
    pub upper_value: Option<String>,
    pub lower_value: Option<String>,
    pub is_read_only: String,
    pub owning_template_parameter: Option<String>,
    pub template_parameter: Option<String>,
    pub deployment: Vec<String>,
    pub template_binding: Vec<String>,
    pub owned_template_signature: Option<String>,
    pub datatype: Option<String>,
    pub is_derived: String,
    pub is_derived_union: String,
    pub aggregation: AggregationKind,
    pub redefined_property: Vec<String>,
    pub owning_association: Option<String>,
    pub default_value: Option<String>,
    pub subsetted_property: Vec<String>,
    pub association: Option<String>,
    pub qualifier: Vec<String>,
    pub association_end: Option<String>,
}

#[wasm_bindgen]
impl ExtensionEnd {
    /// Creates a new ExtensionEnd and returns its ID
    #[wasm_bindgen]
    pub fn create(is_leaf: String, is_static: String, is_ordered: String, is_unique: String, is_read_only: String, is_derived: String, is_derived_union: String, aggregation: AggregationKind) -> String {
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
            is_static: is_static,
            type_: None,
            is_ordered: is_ordered,
            is_unique: is_unique,
            upper_value: None,
            lower_value: None,
            is_read_only: is_read_only,
            owning_template_parameter: None,
            template_parameter: None,
            deployment: Vec::new(),
            template_binding: Vec::new(),
            owned_template_signature: None,
            datatype: None,
            is_derived: is_derived,
            is_derived_union: is_derived_union,
            aggregation: aggregation,
            redefined_property: Vec::new(),
            owning_association: None,
            default_value: None,
            subsetted_property: Vec::new(),
            association: None,
            qualifier: Vec::new(),
            association_end: None,
        };

        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(id.clone(), instance);

        id
    }

    /// Gets a ExtensionEnd by ID
    /// Returns the instance as a JavaScript object
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))
            .and_then(|instance| {
                serde_wasm_bindgen::to_value(instance)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            })
    }

    /// Updates a ExtensionEnd instance
    /// Takes a JavaScript object and updates the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: ExtensionEnd = serde_wasm_bindgen::from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow_mut()
            .insert(instance.id.clone(), instance);

        Ok(())
    }

    /// Deletes a ExtensionEnd by ID
    /// Returns true if deleted, false if not found
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow_mut()
            .remove(&id)
            .is_some()
    }

    /// Checks if a ExtensionEnd exists by ID
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow()
            .contains_key(&id)
    }

    /// Gets all ExtensionEnd instances
    /// Returns an array of JavaScript objects
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        let instances: Vec<ExtensionEnd> = EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow()
            .values()
            .cloned()
            .collect();

        serde_wasm_bindgen::to_value(&instances)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Returns the count of ExtensionEnd instances
    #[wasm_bindgen]
    pub fn count() -> usize {
        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow()
            .len()
    }

    /// Removes all ExtensionEnd instances
    #[wasm_bindgen]
    pub fn clear_all() {
        EXTENSION_END_REGISTRY.lock().unwrap()
            .borrow_mut()
            .clear();
    }

    /// Finds ExtensionEnd instances by name pattern
    /// Returns an array of matching JavaScript objects
    #[wasm_bindgen]
    pub fn find_by_name(pattern: String) -> Result<JsValue, JsValue> {
        let instances: Vec<ExtensionEnd> = EXTENSION_END_REGISTRY.lock().unwrap()
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.type_ = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the upper_value reference
    #[wasm_bindgen]
    pub fn set_upper_value(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.lower_value = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the owning_template_parameter reference
    #[wasm_bindgen]
    pub fn set_owning_template_parameter(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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

    /// Adds a Deployment to deployment
    #[wasm_bindgen]
    pub fn add_deployment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.deployment.contains(&ref_id) {
            return Ok(false);
        }

        instance.deployment.push(ref_id.clone());

        if let Ok(target_js) = Deployment::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Deployment>(target_js) {
                target.location = instance_id;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Deployment::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Deployment from deployment
    #[wasm_bindgen]
    pub fn remove_deployment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.deployment.iter().position(|x| x == &ref_id) {
            instance.deployment.remove(pos);

        if let Ok(target_js) = Deployment::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Deployment>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Deployment::update(target_js);
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

    /// Clears all Deployment from deployment
    #[wasm_bindgen]
    pub fn clear_deployment(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.deployment.len();

        // Update all opposite references
        for ref_id in &instance.deployment {
            if let Ok(target_js) = Deployment::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Deployment>(target_js) {
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Deployment::update(target_js);
                }
            }
        }
        }

        instance.deployment.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Adds a TemplateBinding to template_binding
    #[wasm_bindgen]
    pub fn add_template_binding(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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

    /// Sets the datatype reference
    #[wasm_bindgen]
    pub fn set_datatype(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.datatype {
            if let Ok(target_js) = DataType::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<DataType>(target_js) {
                    target.owned_attribute.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = DataType::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = DataType::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<DataType>(target_js) {
                if !target.owned_attribute.contains(&instance_id) {
                    target.owned_attribute.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = DataType::update(target_js);
                }
            }
        }

        }

        instance.datatype = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a Property to redefined_property
    #[wasm_bindgen]
    pub fn add_redefined_property(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.redefined_property.contains(&ref_id) {
            return Ok(false);
        }

        instance.redefined_property.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Property from redefined_property
    #[wasm_bindgen]
    pub fn remove_redefined_property(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.redefined_property.iter().position(|x| x == &ref_id) {
            instance.redefined_property.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Property from redefined_property
    #[wasm_bindgen]
    pub fn clear_redefined_property(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.redefined_property.len();

        instance.redefined_property.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the owning_association reference
    #[wasm_bindgen]
    pub fn set_owning_association(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.owning_association {
            if let Ok(target_js) = Association::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Association>(target_js) {
                    target.owned_end.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = Association::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = Association::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Association>(target_js) {
                if !target.owned_end.contains(&instance_id) {
                    target.owned_end.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Association::update(target_js);
                }
            }
        }

        }

        instance.owning_association = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the default_value reference
    #[wasm_bindgen]
    pub fn set_default_value(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.default_value = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a Property to subsetted_property
    #[wasm_bindgen]
    pub fn add_subsetted_property(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.subsetted_property.contains(&ref_id) {
            return Ok(false);
        }

        instance.subsetted_property.push(ref_id.clone());

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Property from subsetted_property
    #[wasm_bindgen]
    pub fn remove_subsetted_property(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.subsetted_property.iter().position(|x| x == &ref_id) {
            instance.subsetted_property.remove(pos);

            let updated_js = serde_wasm_bindgen::to_value(&instance)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Self::update(updated_js)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clears all Property from subsetted_property
    #[wasm_bindgen]
    pub fn clear_subsetted_property(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.subsetted_property.len();

        instance.subsetted_property.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the association reference
    #[wasm_bindgen]
    pub fn set_association(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.association {
            if let Ok(target_js) = Association::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Association>(target_js) {
                    target.member_end.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = Association::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = Association::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Association>(target_js) {
                if !target.member_end.contains(&instance_id) {
                    target.member_end.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Association::update(target_js);
                }
            }
        }

        }

        instance.association = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Adds a Property to qualifier
    #[wasm_bindgen]
    pub fn add_qualifier(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if instance.qualifier.contains(&ref_id) {
            return Ok(false);
        }

        instance.qualifier.push(ref_id.clone());

        if let Ok(target_js) = Property::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Property>(target_js) {
                target.association_end = Some(instance_id);
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Property::update(target_js);
                }
            }
        }

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(true)
    }

    /// Removes a Property from qualifier
    #[wasm_bindgen]
    pub fn remove_qualifier(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if let Some(pos) = instance.qualifier.iter().position(|x| x == &ref_id) {
            instance.qualifier.remove(pos);

        if let Ok(target_js) = Property::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Property>(target_js) {
                target.association_end = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Property::update(target_js);
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

    /// Clears all Property from qualifier
    #[wasm_bindgen]
    pub fn clear_qualifier(instance_id: String) -> Result<usize, JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let count = instance.qualifier.len();

        // Update all opposite references
        for ref_id in &instance.qualifier {
            if let Ok(target_js) = Property::get(ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Property>(target_js) {
                target.association_end = None;
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Property::update(target_js);
                }
            }
        }
        }

        instance.qualifier.clear();

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(count)
    }

    /// Sets the association_end reference
    #[wasm_bindgen]
    pub fn set_association_end(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Remove from old opposite
        if let Some(old_ref_id) = &instance.association_end {
            if let Ok(target_js) = Property::get(old_ref_id.clone()) {
                if let Ok(mut target) = serde_wasm_bindgen::from_value::<Property>(target_js) {
                    target.qualifier.retain(|x| x != &instance_id);
                    if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                        let _ = Property::update(target_js);
                    }
                }
            }
        }

        // Add to new opposite
        if let Some(new_ref_id) = &ref_id {
        if let Ok(target_js) = Property::get(new_ref_id.clone()) {
            if let Ok(mut target) = serde_wasm_bindgen::from_value::<Property>(target_js) {
                if !target.qualifier.contains(&instance_id) {
                    target.qualifier.push(instance_id.clone());
                }
                if let Ok(target_js) = serde_wasm_bindgen::to_value(&target) {
                    let _ = Property::update(target_js);
                }
            }
        }

        }

        instance.association_end = ref_id;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the name field
    #[wasm_bindgen]
    pub fn set_name(instance_id: String, value: Option<String>) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_leaf = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_static field
    #[wasm_bindgen]
    pub fn set_is_static(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_static = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_ordered field
    #[wasm_bindgen]
    pub fn set_is_ordered(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
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
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_unique = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_read_only field
    #[wasm_bindgen]
    pub fn set_is_read_only(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_read_only = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_derived field
    #[wasm_bindgen]
    pub fn set_is_derived(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_derived = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the is_derived_union field
    #[wasm_bindgen]
    pub fn set_is_derived_union(instance_id: String, value: String) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.is_derived_union = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

    /// Sets the aggregation field
    #[wasm_bindgen]
    pub fn set_aggregation(instance_id: String, value: AggregationKind) -> Result<(), JsValue> {
        let instance_js = Self::get(instance_id.clone())?;
        let mut instance: ExtensionEnd = serde_wasm_bindgen::from_value(instance_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        instance.aggregation = value;

        let updated_js = serde_wasm_bindgen::to_value(&instance)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Self::update(updated_js)?;

        Ok(())
    }

}

impl ExtensionEnd {
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

        // Validate all deployment references exist
        for id in &self.deployment {
            if !Deployment::exists(id.clone()) {
                errors.push(format!("Deployment {} not found", id));
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

        // Validate datatype reference exists
        if let Some(id) = &self.datatype {
            if !DataType::exists(id.clone()) {
                errors.push(format!("DataType {} not found", id));
            }
        }

        // Validate all redefined_property references exist
        for id in &self.redefined_property {
            if !Property::exists(id.clone()) {
                errors.push(format!("Property {} not found", id));
            }
        }

        // Validate owning_association reference exists
        if let Some(id) = &self.owning_association {
            if !Association::exists(id.clone()) {
                errors.push(format!("Association {} not found", id));
            }
        }

        // Validate default_value reference exists
        if let Some(id) = &self.default_value {
            if !ValueSpecification::exists(id.clone()) {
                errors.push(format!("ValueSpecification {} not found", id));
            }
        }

        // Validate all subsetted_property references exist
        for id in &self.subsetted_property {
            if !Property::exists(id.clone()) {
                errors.push(format!("Property {} not found", id));
            }
        }

        // Validate association reference exists
        if let Some(id) = &self.association {
            if !Association::exists(id.clone()) {
                errors.push(format!("Association {} not found", id));
            }
        }

        // Validate all qualifier references exist
        for id in &self.qualifier {
            if !Property::exists(id.clone()) {
                errors.push(format!("Property {} not found", id));
            }
        }

        // Validate association_end reference exists
        if let Some(id) = &self.association_end {
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

