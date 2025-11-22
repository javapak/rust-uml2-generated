// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           ParameterDirectionKind (enum)
// Source Package: uml
// Package URI:    http://www.eclipse.org/uml2/2.1.0/UML
// Generated:      2025-11-22 12:14:06
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

use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum ParameterDirectionKind {
    In,
    Inout,
    Out,
    Return,
}

impl std::fmt::Display for ParameterDirectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In => write!(f, "in"),
            Self::Inout => write!(f, "inout"),
            Self::Out => write!(f, "out"),
            Self::Return => write!(f, "return"),
        }
    }
}

