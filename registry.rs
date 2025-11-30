// ═══════════════════════════════════════════════════════════════
// registry.rs - Unified Registry System
// Generated code - do not modify manually
// ═══════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;

/// A registry entry with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub data: serde_json::Value,
}

lazy_static::lazy_static! {
    static ref GLOBAL_REGISTRY: Mutex<RefCell<HashMap<String, RegistryEntry>>> =
        Mutex::new(RefCell::new(HashMap::new()));
}

/// Insert any serializable instance into the registry
pub fn insert<T: Serialize>(id: String, type_name: &str, instance: &T) -> Result<(), JsValue> {
    let data = serde_json::to_value(instance)
            .map_err(|_| JsValue::from_str("Invalid data"))?;
    
    let entry = RegistryEntry {
        id: id.clone(),
        type_name: type_name.to_string(),
        data,
    };
    
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow_mut()
        .insert(id, entry);
    
    Ok(())
}

/// Get and deserialize an instance by ID
pub fn get<T: for<'de> Deserialize<'de>>(id: &str) -> Option<T> {
    get_as_jsvalue(id)
    .ok()
    .and_then(|jsval| serde_wasm_bindgen::from_value(jsval).ok())
}

/// Get instance as JsValue (for WASM boundary)
pub fn get_as_jsvalue(id: &str) -> Result<JsValue, JsValue> {
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow()
        .get(id)
        .ok_or_else(|| JsValue::from_str("Not found"))
        .and_then(|entry| {
            serde_wasm_bindgen::to_value(&entry.data)
            .map_err(|_| JsValue::from_str("Invalid data"))
        })
}

/// Update from JsValue
pub fn update_from_jsvalue(id: String, type_name: &str, value: JsValue) -> Result<(), JsValue> {
    let data: serde_json::Value = serde_wasm_bindgen::from_value(value)
            .map_err(|_| JsValue::from_str("Invalid data"))?;
    
    let entry = RegistryEntry {
        id: id.clone(),
        type_name: type_name.to_string(),
        data,
    };
    
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow_mut()
        .insert(id, entry);
    
    Ok(())
}

/// Delete an instance by ID
pub fn delete(id: &str) -> bool {
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow_mut()
        .remove(id)
        .is_some()
}

/// Check if an instance exists
pub fn exists(id: &str) -> bool {
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow()
        .contains_key(id)
}

/// Get all instances of a specific type
pub fn get_all_of_type(type_name: &str) -> Vec<serde_json::Value> {
    GLOBAL_REGISTRY
        .lock()
        .unwrap()
        .borrow()
        .values()
        .filter(|entry| entry.type_name == type_name)
        .map(|entry| entry.data.clone())
        .collect()
}

/// Get all instances of a specific type as JsValue
pub fn get_all_of_type_as_jsvalue(type_name: &str) -> Result<JsValue, JsValue> {
    let instances = get_all_of_type(type_name);
    serde_wasm_bindgen::to_value(&instances)
            .map_err(|_| JsValue::from_str("Invalid data"))
}

/// Clear all instances of a specific type
pub fn clear_type(type_name: &str) -> usize {
    let registry = GLOBAL_REGISTRY.lock().unwrap();
    let mut map = registry.borrow_mut();
    
    let to_remove: Vec<String> = map
        .values()
        .filter(|entry| entry.type_name == type_name)
        .map(|entry| entry.id.clone())
        .collect();
    
    let count = to_remove.len();
    for id in to_remove {
        map.remove(&id);
    }
    
    count
}

/// Clear all instances from the registry
pub fn clear_all() -> usize {
    let registry = GLOBAL_REGISTRY.lock().unwrap();
    let count = registry.borrow().len();
    registry.borrow_mut().clear();
    count
}

/// Get all type names currently in the registry
pub fn get_all_types() -> Vec<String> {
    let registry = GLOBAL_REGISTRY.lock().unwrap();
    let mut types: Vec<String> = registry
        .borrow()
        .values()
        .map(|entry| entry.type_name.clone())
        .collect();
    
    types.sort();
    types.dedup();
    types
}
