// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ClassifierTemplateParameter (struct)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-22 12:14:07
// Generator:      EcoreToRustGenerator v0.1.0
//
// Generation Options:
//   - WASM:       enabled
//   - Tsify:      disabled
//   - Serde:      enabled
//   - Builders:   disabled
//   - References: String IDs
//
// WARNING: This file is auto-generated. Manual changes will be overwritten.
// ============================================================================

use crate::eannotation::EAnnotation;
use crate::comment::Comment;
use crate::parameterable_element::ParameterableElement;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ClassifierTemplateParameter {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    signature: String,
    parametered_element: String,
    owned_parametered_element: Option<ParameterableElement>,
    default: Option<String>,
    owned_default: Option<ParameterableElement>,
    allow_substitutable: bool,
    default_classifier: Option<String>,
    constraining_classifier: Option<String>,
}

#[wasm_bindgen]
impl ClassifierTemplateParameter {
    pub fn new(signature: String, parametered_element: String, allow_substitutable: bool) -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: signature,
            parametered_element: parametered_element,
            owned_parametered_element: None,
            default: None,
            owned_default: None,
            allow_substitutable: allow_substitutable,
            default_classifier: None,
            constraining_classifier: None,
        }
    }

    /// Returns a clone of signature
    pub fn signature(&self) -> String {
        self.signature.clone()
    }

    /// Sets signature
    pub fn set_signature(&mut self, value: String) {
        self.signature = value;
    }

    /// Takes ownership of signature, replacing it with an empty string
    pub fn take_signature(&mut self) -> String {
        std::mem::take(&mut self.signature)
    }

    /// Returns a clone of parametered_element
    pub fn parametered_element(&self) -> String {
        self.parametered_element.clone()
    }

    /// Sets parametered_element
    pub fn set_parametered_element(&mut self, value: String) {
        self.parametered_element = value;
    }

    /// Takes ownership of parametered_element, replacing it with an empty string
    pub fn take_parametered_element(&mut self) -> String {
        std::mem::take(&mut self.parametered_element)
    }

    /// Returns a clone of default if present
    pub fn default(&self) -> Option<String> {
        self.default.clone()
    }

    /// Sets default
    pub fn set_default(&mut self, value: String) {
        self.default = Some(value);
    }

    /// Takes default, leaving None in its place
    pub fn take_default(&mut self) -> Option<String> {
        self.default.take()
    }

    /// Returns allow_substitutable
    pub fn allow_substitutable(&self) -> bool {
        self.allow_substitutable
    }

    /// Sets allow_substitutable
    pub fn set_allow_substitutable(&mut self, value: bool) {
        self.allow_substitutable = value;
    }

    /// Returns a clone of default_classifier if present
    pub fn default_classifier(&self) -> Option<String> {
        self.default_classifier.clone()
    }

    /// Sets default_classifier
    pub fn set_default_classifier(&mut self, value: String) {
        self.default_classifier = Some(value);
    }

    /// Takes default_classifier, leaving None in its place
    pub fn take_default_classifier(&mut self) -> Option<String> {
        self.default_classifier.take()
    }

    /// Returns a clone of constraining_classifier if present
    pub fn constraining_classifier(&self) -> Option<String> {
        self.constraining_classifier.clone()
    }

    /// Sets constraining_classifier
    pub fn set_constraining_classifier(&mut self, value: String) {
        self.constraining_classifier = Some(value);
    }

    /// Takes constraining_classifier, leaving None in its place
    pub fn take_constraining_classifier(&mut self) -> Option<String> {
        self.constraining_classifier.take()
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|e| e.to_string())
    }

    /// Deserialize from JSON string
    pub fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
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
        "ClassifierTemplateParameter".to_string()
    }

}

impl Default for ClassifierTemplateParameter {
    fn default() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            signature: Default::default(),
            parametered_element: Default::default(),
            owned_parametered_element: None,
            default: None,
            owned_default: None,
            allow_substitutable: false,
            default_classifier: None,
            constraining_classifier: None,
        }
    }
}

impl std::fmt::Display for ClassifierTemplateParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassifierTemplateParameter(...)")
    }
}

