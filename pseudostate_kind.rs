// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           PseudostateKind (enum)
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
pub enum PseudostateKind {
    Initial,
    Deephistory,
    Shallowhistory,
    Join,
    Fork,
    Junction,
    Choice,
    Entrypoint,
    Exitpoint,
    Terminate,
}

impl std::fmt::Display for PseudostateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initial => write!(f, "initial"),
            Self::Deephistory => write!(f, "deepHistory"),
            Self::Shallowhistory => write!(f, "shallowHistory"),
            Self::Join => write!(f, "join"),
            Self::Fork => write!(f, "fork"),
            Self::Junction => write!(f, "junction"),
            Self::Choice => write!(f, "choice"),
            Self::Entrypoint => write!(f, "entryPoint"),
            Self::Exitpoint => write!(f, "exitPoint"),
            Self::Terminate => write!(f, "terminate"),
        }
    }
}

