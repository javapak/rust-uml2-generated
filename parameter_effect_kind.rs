// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ParameterEffectKind (enum)
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

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use tsify::Tsify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum ParameterEffectKind {
    Create,
    Read,
    Update,
    Delete,
}

impl std::fmt::Display for ParameterEffectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => write!(f, "create"),
            Self::Read => write!(f, "read"),
            Self::Update => write!(f, "update"),
            Self::Delete => write!(f, "delete"),
        }
    }
}

