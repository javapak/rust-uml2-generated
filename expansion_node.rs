// ExpansionNode - Generated UML Class
// Uses unified registry with type filtering

use uuid::Uuid;
use crate::registry;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::visibility_kind::VisibilityKind;
use crate::object_node_ordering_kind::ObjectNodeOrderingKind;
use crate::expansion_region::ExpansionRegion;


const TYPE_NAME: &str = "ExpansionNode";

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct ExpansionNode {
    /// Unique identifier for this instance
    #[wasm_bindgen(skip)]
    pub id: String,
    #[wasm_bindgen(skip)]
    pub e_annotations: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_comment: Vec<String>,
    #[wasm_bindgen(skip)]
    pub name: Option<String>,
    #[wasm_bindgen(skip)]
    pub visibility: Option<VisibilityKind>,
    #[wasm_bindgen(skip)]
    pub client_dependency: Vec<String>,
    #[wasm_bindgen(skip)]
    pub name_expression: Option<String>,
    #[wasm_bindgen(skip)]
    pub is_leaf: String,
    #[wasm_bindgen(skip)]
    pub in_structured_node: Option<String>,
    #[wasm_bindgen(skip)]
    pub activity: Option<String>,
    #[wasm_bindgen(skip)]
    pub outgoing: Vec<String>,
    #[wasm_bindgen(skip)]
    pub incoming: Vec<String>,
    #[wasm_bindgen(skip)]
    pub in_partition: Vec<String>,
    #[wasm_bindgen(skip)]
    pub in_interruptible_region: Vec<String>,
    #[wasm_bindgen(skip)]
    pub redefined_node: Vec<String>,
    #[wasm_bindgen(skip)]
    pub type_: Option<String>,
    #[wasm_bindgen(skip)]
    pub ordering: ObjectNodeOrderingKind,
    #[wasm_bindgen(skip)]
    pub is_control_type: String,
    #[wasm_bindgen(skip)]
    pub upper_bound: String,
    #[wasm_bindgen(skip)]
    pub in_state: Vec<String>,
    #[wasm_bindgen(skip)]
    pub selection: Option<String>,
    #[wasm_bindgen(skip)]
    pub region_as_output: Option<String>,
    #[wasm_bindgen(skip)]
    pub region_as_input: Option<String>,
}

#[wasm_bindgen]
impl ExpansionNode {
    /// Creates a new ExpansionNode instance
    #[wasm_bindgen]
    pub fn create(is_leaf: String, ordering: ObjectNodeOrderingKind, is_control_type: String, upper_bound: String) -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            is_leaf,
            in_structured_node: None,
            activity: None,
            outgoing: Vec::new(),
            incoming: Vec::new(),
            in_partition: Vec::new(),
            in_interruptible_region: Vec::new(),
            redefined_node: Vec::new(),
            type_: None,
            ordering,
            is_control_type,
            upper_bound,
            in_state: Vec::new(),
            selection: None,
            region_as_output: None,
            region_as_input: None,
        };
        
        registry::insert(id.clone(), TYPE_NAME, &instance)
            .expect("Failed to insert into registry");
        
        id
    }

    /// Gets a snapshot of this instance
    /// Note: Returns a snapshot. Modifications require calling update().
    #[wasm_bindgen]
    pub fn get(id: String) -> Result<JsValue, JsValue> {
        registry::get_as_jsvalue(&id)
    }

    /// Updates the instance in the registry
    #[wasm_bindgen]
    pub fn update(value: JsValue) -> Result<(), JsValue> {
        let instance: ExpansionNode = serde_wasm_bindgen::from_value(value)
            .map_err(|_| JsValue::from_str("Invalid data"))?;
        registry::update_from_jsvalue(
            instance.id.clone(),
            TYPE_NAME,
            serde_wasm_bindgen::to_value(&instance)?
        )
    }

    /// Deletes this instance from the registry
    #[wasm_bindgen]
    pub fn delete(id: String) -> bool {
        registry::delete(&id)
    }

    /// Checks if an instance exists
    #[wasm_bindgen]
    pub fn exists(id: String) -> bool {
        registry::exists(&id)
    }

    /// Gets all instances of this type
    #[wasm_bindgen]
    pub fn get_all() -> Result<JsValue, JsValue> {
        registry::get_all_of_type_as_jsvalue(TYPE_NAME)
    }

    /// Clears all instances of this type
    #[wasm_bindgen]
    pub fn clear_all() -> usize {
        registry::clear_type(TYPE_NAME)
    }

#[wasm_bindgen]
pub fn set_name(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.name = Some(value);
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn clear_name(instance_id: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.name = None;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_visibility(instance_id: String, value: VisibilityKind) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.visibility = Some(value);
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn clear_visibility(instance_id: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.visibility = None;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_is_leaf(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.is_leaf = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_ordering(instance_id: String, value: ObjectNodeOrderingKind) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.ordering = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_is_control_type(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.is_control_type = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}





    #[wasm_bindgen]
    pub fn set_region_as_output(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if let Some(old_ref_id) = &instance.region_as_output {
            if let Some(mut old_target) = registry::get::<ExpansionRegion>(&old_ref_id) {
                old_target.output_element.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "ExpansionRegion", &old_target);
            }
        }
        
        instance.region_as_output = ref_id.clone();
        
        if let Some(new_ref_id) = &ref_id {
            if let Some(mut new_target) = registry::get::<ExpansionRegion>(&new_ref_id) {
                if !new_target.output_element.contains(&instance_id) {
                    new_target.output_element.push(instance_id.clone());
                }
                let _ = registry::insert(new_ref_id.clone(), "ExpansionRegion", &new_target);
            }
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear_region_as_output(instance_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.region_as_output.is_none() {
            return Ok(false);
        }
        
        if let Some(old_ref_id) = &instance.region_as_output {
            if let Some(mut old_target) = registry::get::<ExpansionRegion>(&old_ref_id) {
                old_target.output_element.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "ExpansionRegion", &old_target);
            }
        }
        
        instance.region_as_output = None;
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn set_region_as_input(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if let Some(old_ref_id) = &instance.region_as_input {
            if let Some(mut old_target) = registry::get::<ExpansionRegion>(&old_ref_id) {
                old_target.input_element.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "ExpansionRegion", &old_target);
            }
        }
        
        instance.region_as_input = ref_id.clone();
        
        if let Some(new_ref_id) = &ref_id {
            if let Some(mut new_target) = registry::get::<ExpansionRegion>(&new_ref_id) {
                if !new_target.input_element.contains(&instance_id) {
                    new_target.input_element.push(instance_id.clone());
                }
                let _ = registry::insert(new_ref_id.clone(), "ExpansionRegion", &new_target);
            }
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear_region_as_input(instance_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.region_as_input.is_none() {
            return Ok(false);
        }
        
        if let Some(old_ref_id) = &instance.region_as_input {
            if let Some(mut old_target) = registry::get::<ExpansionRegion>(&old_ref_id) {
                old_target.input_element.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "ExpansionRegion", &old_target);
            }
        }
        
        instance.region_as_input = None;
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }


    /// Returns whether this type can be created standalone (not nested)
    pub fn can_exist_standalone() -> bool {
        true
    }

    /// Returns whether this type requires a container
    pub fn requires_container() -> bool {
        false
    }

    /// Returns the type name
    pub fn type_name() -> String {
        "ExpansionNode".to_string()
    }


}
