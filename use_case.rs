// UseCase - Generated UML Class
// Uses unified registry with type filtering

use uuid::Uuid;
use crate::registry;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use tsify::Tsify;
use crate::visibility_kind::VisibilityKind;
use crate::classifier::Classifier;
use crate::include::Include;
use crate::extend::Extend;
use crate::extension_point::ExtensionPoint;


const TYPE_NAME: &str = "UseCase";

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct UseCase {
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
    pub owned_behavior: Vec<String>,
    #[wasm_bindgen(skip)]
    pub classifier_behavior: Option<String>,
    #[wasm_bindgen(skip)]
    pub interface_realization: Vec<String>,
    #[wasm_bindgen(skip)]
    pub owned_trigger: Vec<String>,
    #[wasm_bindgen(skip)]
    pub include: Vec<String>,
    #[wasm_bindgen(skip)]
    pub extend: Vec<String>,
    #[wasm_bindgen(skip)]
    pub extension_point: Vec<String>,
    #[wasm_bindgen(skip)]
    pub subject: Vec<String>,
}

#[wasm_bindgen]
impl UseCase {
    /// Creates a new UseCase instance
    #[wasm_bindgen]
    pub fn create(is_leaf: String, is_abstract: String) -> String {
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
            owned_behavior: Vec::new(),
            classifier_behavior: None,
            interface_realization: Vec::new(),
            owned_trigger: Vec::new(),
            include: Vec::new(),
            extend: Vec::new(),
            extension_point: Vec::new(),
            subject: Vec::new(),
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
        let instance: UseCase = serde_wasm_bindgen::from_value(value)
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
    pub fn add_include(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.include.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.include.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Include>(&ref_id) {
            target.including_case = instance_id.clone();
            let _ = registry::insert(ref_id.clone(), "Include", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_include(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.include.len();
        instance.include.retain(|id| id != &ref_id);
        let removed = instance.include.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(target) = registry::get::<Include>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Include", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_include(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.include.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.include {
        // Update opposite reference
        if let Some(target) = registry::get::<Include>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Include", &target);
        }
        }
        
        instance.include.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_extend(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.extend.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.extend.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Extend>(&ref_id) {
            target.extension = instance_id.clone();
            let _ = registry::insert(ref_id.clone(), "Extend", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_extend(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.extend.len();
        instance.extend.retain(|id| id != &ref_id);
        let removed = instance.extend.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(target) = registry::get::<Extend>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Extend", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_extend(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.extend.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.extend {
        // Update opposite reference
        if let Some(target) = registry::get::<Extend>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "Extend", &target);
        }
        }
        
        instance.extend.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_extension_point(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.extension_point.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.extension_point.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<ExtensionPoint>(&ref_id) {
            target.use_case = instance_id.clone();
            let _ = registry::insert(ref_id.clone(), "ExtensionPoint", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_extension_point(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.extension_point.len();
        instance.extension_point.retain(|id| id != &ref_id);
        let removed = instance.extension_point.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(target) = registry::get::<ExtensionPoint>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "ExtensionPoint", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_extension_point(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.extension_point.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.extension_point {
        // Update opposite reference
        if let Some(target) = registry::get::<ExtensionPoint>(&ref_id) {
            let _ = registry::insert(ref_id.clone(), "ExtensionPoint", &target);
        }
        }
        
        instance.extension_point.clear();
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(count)
    }

    #[wasm_bindgen]
    pub fn add_subject(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        if instance.subject.contains(&ref_id) {
            return Ok(false);
        }
        
        instance.subject.push(ref_id.clone());
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Classifier>(&ref_id) {
            if !target.use_case.contains(&instance_id) {
                target.use_case.push(instance_id.clone());
            }
            let _ = registry::insert(ref_id.clone(), "Classifier", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn remove_subject(instance_id: String, ref_id: String) -> Result<bool, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let initial_len = instance.subject.len();
        instance.subject.retain(|id| id != &ref_id);
        let removed = instance.subject.len() < initial_len;
        
        if !removed {
            return Ok(false);
        }
        
        // Update opposite reference
        if let Some(mut target) = registry::get::<Classifier>(&ref_id) {
            target.use_case.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "Classifier", &target);
        }
        
        registry::insert(instance_id, TYPE_NAME, &instance)?;
        
        Ok(true)
    }

    #[wasm_bindgen]
    pub fn clear_subject(instance_id: String) -> Result<usize, JsValue> {
        let mut instance: Self = registry::get(&instance_id)
            .ok_or_else(|| JsValue::from_str("Instance not found"))?;
        
        let count = instance.subject.len();
        
        if count == 0 {
            return Ok(0);
        }
        
        for ref_id in &instance.subject {
        // Update opposite reference
        if let Some(mut target) = registry::get::<Classifier>(&ref_id) {
            target.use_case.retain(|id| id != &instance_id);
            let _ = registry::insert(ref_id.clone(), "Classifier", &target);
        }
        }
        
        instance.subject.clear();
        
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
        "UseCase".to_string()
    }


}
