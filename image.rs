// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           Image (struct)
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
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[wasm_bindgen]
pub struct Image {
    e_annotations: Vec<EAnnotation>,
    owned_comment: Vec<Comment>,
    content: Option<String>,
    location: Option<String>,
    format: Option<String>,
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Self {
        Self {
            e_annotations: Vec::new(),
            owned_comment: Vec::new(),
            content: None,
            location: None,
            format: None,
        }
    }

    /// Returns a clone of content if present
    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    /// Sets content
    pub fn set_content(&mut self, value: String) {
        self.content = Some(value);
    }

    /// Takes content, leaving None in its place
    pub fn take_content(&mut self) -> Option<String> {
        self.content.take()
    }

    /// Returns a clone of location if present
    pub fn location(&self) -> Option<String> {
        self.location.clone()
    }

    /// Sets location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Takes location, leaving None in its place
    pub fn take_location(&mut self) -> Option<String> {
        self.location.take()
    }

    /// Returns a clone of format if present
    pub fn format(&self) -> Option<String> {
        self.format.clone()
    }

    /// Sets format
    pub fn set_format(&mut self, value: String) {
        self.format = Some(value);
    }

    /// Takes format, leaving None in its place
    pub fn take_format(&mut self) -> Option<String> {
        self.format.take()
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
        "Image".to_string()
    }

}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image(...)")
    }
}

