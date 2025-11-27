// InteractionFragment - Generated UML Class
// Uses unified registry with type filtering

use uuid::Uuid;
use crate::registry;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::visibility_kind::VisibilityKind;
use crate::lifeline::Lifeline;
use crate::interaction::Interaction;
use crate::interaction_operand::InteractionOperand;


const TYPE_NAME: &str = "InteractionFragment";

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct InteractionFragment {
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
    pub covered: Vec<String>,
    #[wasm_bindgen(skip)]
    pub general_ordering: Vec<String>,
    #[wasm_bindgen(skip)]
    pub enclosing_interaction: Option<String>,
    #[wasm_bindgen(skip)]
    pub enclosing_operand: Option<String>,
}

#[wasm_bindgen]
impl InteractionFragment {
    /// Creates a new InteractionFragment instance
    #[wasm_bindgen]
    pub fn create() -> String {
        let id = Uuid::new_v4().to_string();
        let instance = Self {
            id: id.clone(),
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            name: None,
            visibility: None,
            client_dependency: Vec::new(),
            name_expression: None,
            covered: Vec::new(),
            general_ordering: Vec::new(),
            enclosing_interaction: None,
            enclosing_operand: None,
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
        let instance: InteractionFragment = serde_wasm_bindgen::from_value(value)
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
    pub fn add_covered(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.covered.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.covered.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Lifeline>(&ref_id) {
            if !target.covered_by.contains(&instance_id) {
                target.covered_by.push(instance_id.clone());
            }
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_covered(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.covered.len();
        instance.covered.retain(|id| id != &ref_id);
        let removed = instance.covered.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Lifeline>(&ref_id) {
            target.covered_by.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_covered(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.covered.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.covered {
        // Update opposite reference
        if let Some(mut target) = registry::get::<Lifeline>(&ref_id) {
            target.covered_by.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        }
        
        instance.covered.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_general_ordering(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.general_ordering.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.general_ordering.push(ref_id.clone());
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_general_ordering(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.general_ordering.len();
        instance.general_ordering.retain(|id| id != &ref_id);
        let removed = instance.general_ordering.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_general_ordering(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.general_ordering.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        instance.general_ordering.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn set_enclosing_interaction(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if let Some(old_ref_id) = &instance.enclosing_interaction {
            if let Some(mut old_target) = registry::get::<Interaction>(&old_ref_id) {
                old_target.fragment.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "Interaction", &old_target);
            }
        }
        
        instance.enclosing_interaction = ref_id.clone();
        
        if let Some(new_ref_id) = &ref_id {
            if let Some(mut new_target) = registry::get::<Interaction>(&new_ref_id) {
                if !new_target.fragment.contains(&instance_id) {
                    new_target.fragment.push(instance_id.clone());
                }
                let _ = registry::insert(new_ref_id.clone(), "Interaction", &new_target);
            }
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear_enclosing_interaction(instance_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.enclosing_interaction.is_none() {
            return Ok(false);
        }
        
        if let Some(old_ref_id) = &instance.enclosing_interaction {
            if let Some(mut old_target) = registry::get::<Interaction>(&old_ref_id) {
                old_target.fragment.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "Interaction", &old_target);
            }
        }
        
        instance.enclosing_interaction = None;
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn set_enclosing_operand(instance_id: String, ref_id: Option<String>) -> Result<(), JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if let Some(old_ref_id) = &instance.enclosing_operand {
            if let Some(mut old_target) = registry::get::<InteractionOperand>(&old_ref_id) {
                old_target.fragment.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "InteractionOperand", &old_target);
            }
        }
        
        instance.enclosing_operand = ref_id.clone();
        
        if let Some(new_ref_id) = &ref_id {
            if let Some(mut new_target) = registry::get::<InteractionOperand>(&new_ref_id) {
                if !new_target.fragment.contains(&instance_id) {
                    new_target.fragment.push(instance_id.clone());
                }
                let _ = registry::insert(new_ref_id.clone(), "InteractionOperand", &new_target);
            }
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear_enclosing_operand(instance_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.enclosing_operand.is_none() {
            return Ok(false);
        }
        
        if let Some(old_ref_id) = &instance.enclosing_operand {
            if let Some(mut old_target) = registry::get::<InteractionOperand>(&old_ref_id) {
                old_target.fragment.retain(|id| id != &instance_id);
                let _ = registry::insert(old_ref_id.clone(), "InteractionOperand", &old_target);
            }
        }
        
        instance.enclosing_operand = None;
        
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
        "InteractionFragment".to_string()
    }


}
