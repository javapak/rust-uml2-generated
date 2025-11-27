// InterruptibleActivityRegion - Generated UML Class
// Uses unified registry with type filtering

use uuid::Uuid;
use crate::registry;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::activity_node::ActivityNode;
use crate::activity_edge::ActivityEdge;


const TYPE_NAME: &str = "InterruptibleActivityRegion";

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct InterruptibleActivityRegion {
    /// Unique identifier for this instance
    #[wasm_bindgen(skip)]
    pub id: String,
    #[wasm_bindgen(skip)]
    pub e_annotations: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_comment: Vec<String>,
    #[wasm_bindgen(skip)]
    pub in_activity: Option<String>,
    #[wasm_bindgen(skip)]
    pub node: Vec<String>,
    #[wasm_bindgen(skip)]
    pub interrupting_edge: Vec<String>,
}

#[wasm_bindgen]
impl InterruptibleActivityRegion {
    /// Creates a new InterruptibleActivityRegion instance
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
        let instance: InterruptibleActivityRegion = serde_wasm_bindgen::from_value(value)
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
    pub fn add_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.node.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.node.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityNode>(&ref_id) {
            if !target.in_interruptible_region.contains(&instance_id) {
                target.in_interruptible_region.push(instance_id.clone());
            }
            let _ = registry::insert(ref_id.clone(), "ActivityNode", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_node(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.node.len();
        instance.node.retain(|id| id != &ref_id);
        let removed = instance.node.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityNode>(&ref_id) {
            target.in_interruptible_region.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "ActivityNode", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_node(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.node.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.node {
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityNode>(&ref_id) {
            target.in_interruptible_region.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "ActivityNode", &target);
        }
        }
        
        instance.node.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_interrupting_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.interrupting_edge.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.interrupting_edge.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityEdge>(&ref_id) {
            target.interrupts = Some(instance_id.clone());
            let _ = registry::insert(ref_id.clone(), "ActivityEdge", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_interrupting_edge(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.interrupting_edge.len();
        instance.interrupting_edge.retain(|id| id != &ref_id);
        let removed = instance.interrupting_edge.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityEdge>(&ref_id) {
            target.interrupts = None;
            let _ = registry::insert(ref_id.clone(), "ActivityEdge", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_interrupting_edge(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.interrupting_edge.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.interrupting_edge {
        // Update opposite reference
        if let Some(mut target) = registry::get::<ActivityEdge>(&ref_id) {
            target.interrupts = None;
            let _ = registry::insert(ref_id.clone(), "ActivityEdge", &target);
        }
        }
        
        instance.interrupting_edge.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
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
        "InterruptibleActivityRegion".to_string()
    }


}
