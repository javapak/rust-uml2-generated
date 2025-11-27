// Interaction - Generated UML Class
// Uses unified registry with type filtering

use uuid::Uuid;
use crate::registry;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::visibility_kind::VisibilityKind;
use crate::lifeline::Lifeline;
use crate::interaction_fragment::InteractionFragment;
use crate::message::Message;


const TYPE_NAME: &str = "Interaction";

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Interaction {
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
    pub element_import: Vec<String>,
    #[wasm_bindgen(skip)]
    pub package_import: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_rule: Vec<String>,
    #[wasm_bindgen(skip)]
    pub is_leaf: String,
    #[wasm_bindgen(skip)]
    pub owning_template_parameter: Option<String>,
    #[wasm_bindgen(skip)]
    pub template_parameter: Option<String>,
    #[wasm_bindgen(skip)]
    pub template_binding: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_template_signature: Option<String>,
    #[wasm_bindgen(skip)]
    pub is_abstract: String,
    #[wasm_bindgen(skip)]
    pub generalization: Vec<String>,
    #[wasm_bindgen(skip)]
    pub powertype_extent: Vec<String>,
    #[wasm_bindgen(skip)]
    pub redefined_classifier: Vec<String>,
    #[wasm_bindgen(skip)]
    pub substitution: Vec<String>,
    #[wasm_bindgen(skip)]
    pub representation: Option<String>,
    #[wasm_bindgen(skip)]
    pub collaboration_use: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_use_case: Vec<String>,
    #[wasm_bindgen(skip)]
    pub use_case: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_attribute: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_connector: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_behavior: Vec<String>,
    #[wasm_bindgen(skip)]
    pub classifier_behavior: Option<String>,
    #[wasm_bindgen(skip)]
    pub interface_realization: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_trigger: Vec<String>,
    #[wasm_bindgen(skip)]
    pub nested_classifier: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_operation: Vec<String>,
    #[wasm_bindgen(skip)]
    pub is_active: String,
    #[wasm_bindgen(skip)]
    pub owned_reception: Vec<String>,
    #[wasm_bindgen(skip)]
    pub is_reentrant: String,
    #[wasm_bindgen(skip)]
    pub redefined_behavior: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_parameter: Vec<String>,
    #[wasm_bindgen(skip)]
    pub precondition: Vec<String>,
    #[wasm_bindgen(skip)]
    pub postcondition: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_parameter_set: Vec<String>,
    #[wasm_bindgen(skip)]
    pub specification: Option<String>,
    #[wasm_bindgen(skip)]
    pub covered: Vec<String>,
    #[wasm_bindgen(skip)]
    pub general_ordering: Vec<String>,
    #[wasm_bindgen(skip)]
    pub enclosing_interaction: Option<String>,
    #[wasm_bindgen(skip)]
    pub enclosing_operand: Option<String>,
    #[wasm_bindgen(skip)]
    pub lifeline: Vec<String>,
    #[wasm_bindgen(skip)]
    pub fragment: Vec<String>,
    #[wasm_bindgen(skip)]
    pub action: Vec<String>,
    #[wasm_bindgen(skip)]
    pub formal_gate: Vec<String>,
    #[wasm_bindgen(skip)]
    pub message: Vec<String>,
}

#[wasm_bindgen]
impl Interaction {
    /// Creates a new Interaction instance
    #[wasm_bindgen]
    pub fn create(is_leaf: String, is_abstract: String, is_active: String, is_reentrant: String) -> String {
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
            is_leaf,
            owning_template_parameter: None,
            template_parameter: None,
            template_binding: Vec::new(),
            owned_template_signature: None,
            is_abstract,
            generalization: Vec::new(),
            powertype_extent: Vec::new(),
            redefined_classifier: Vec::new(),
            substitution: Vec::new(),
            representation: None,
            collaboration_use: Vec::new(),
            owned_use_case: Vec::new(),
            use_case: Vec::new(),
            owned_attribute: Vec::new(),
            owned_connector: Vec::new(),
            owned_behavior: Vec::new(),
            classifier_behavior: None,
            interface_realization: Vec::new(),
            owned_trigger: Vec::new(),
            nested_classifier: Vec::new(),
            owned_operation: Vec::new(),
            is_active,
            owned_reception: Vec::new(),
            is_reentrant,
            redefined_behavior: Vec::new(),
            owned_parameter: Vec::new(),
            precondition: Vec::new(),
            postcondition: Vec::new(),
            owned_parameter_set: Vec::new(),
            specification: None,
            covered: Vec::new(),
            general_ordering: Vec::new(),
            enclosing_interaction: None,
            enclosing_operand: None,
            lifeline: Vec::new(),
            fragment: Vec::new(),
            action: Vec::new(),
            formal_gate: Vec::new(),
            message: Vec::new(),
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
        let instance: Interaction = serde_wasm_bindgen::from_value(value)
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
pub fn set_is_abstract(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.is_abstract = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_is_active(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.is_active = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_is_reentrant(instance_id: String, value: String) -> Result<(), JsValue> {
    let mut instance: Self = registry::get(&instance_id)
        .ok_or_else(|| JsValue::from_str("Instance not found"))?;
    instance.is_reentrant = value;
    registry::insert(instance_id, TYPE_NAME, &instance)?;
    Ok(())
}





    #[wasm_bindgen]
    pub fn add_lifeline(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.lifeline.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.lifeline.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Lifeline>(&ref_id) {
            target.interaction = instance_id.clone();
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_lifeline(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.lifeline.len();
        instance.lifeline.retain(|id| id != &ref_id);
        let removed = instance.lifeline.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(target) = registry::get::<Lifeline>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_lifeline(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.lifeline.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.lifeline {
        // Update opposite reference
        if let Some(target) = registry::get::<Lifeline>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Lifeline", &target);
        }
        }
        
        instance.lifeline.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_fragment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.fragment.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.fragment.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<InteractionFragment>(&ref_id) {
            target.enclosing_interaction = Some(instance_id.clone());
            let _ = registry::insert(ref_id.clone(), "InteractionFragment", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_fragment(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.fragment.len();
        instance.fragment.retain(|id| id != &ref_id);
        let removed = instance.fragment.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<InteractionFragment>(&ref_id) {
            target.enclosing_interaction = None;
            let _ = registry::insert(ref_id.clone(), "InteractionFragment", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_fragment(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.fragment.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.fragment {
        // Update opposite reference
        if let Some(mut target) = registry::get::<InteractionFragment>(&ref_id) {
            target.enclosing_interaction = None;
            let _ = registry::insert(ref_id.clone(), "InteractionFragment", &target);
        }
        }
        
        instance.fragment.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_action(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.action.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.action.push(ref_id.clone());
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_action(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.action.len();
        instance.action.retain(|id| id != &ref_id);
        let removed = instance.action.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_action(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.action.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        instance.action.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_formal_gate(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.formal_gate.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.formal_gate.push(ref_id.clone());
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_formal_gate(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.formal_gate.len();
        instance.formal_gate.retain(|id| id != &ref_id);
        let removed = instance.formal_gate.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_formal_gate(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.formal_gate.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        instance.formal_gate.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_message(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.message.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.message.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Message>(&ref_id) {
            target.interaction = instance_id.clone();
            let _ = registry::insert(ref_id.clone(), "Message", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_message(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.message.len();
        instance.message.retain(|id| id != &ref_id);
        let removed = instance.message.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(target) = registry::get::<Message>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Message", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_message(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.message.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.message {
        // Update opposite reference
        if let Some(target) = registry::get::<Message>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Message", &target);
        }
        }
        
        instance.message.clear();
        
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
        "Interaction".to_string()
    }


}
