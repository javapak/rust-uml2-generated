// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           InteractionOperatorKind (enum)
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
pub enum InteractionOperatorKind {
    Seq,
    Alt,
    Opt,
    Break,
    Par,
    Strict,
    Loop,
    Critical,
    Neg,
    Assert,
    Ignore,
    Consider,
}

impl std::fmt::Display for InteractionOperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seq => write!(f, "seq"),
            Self::Alt => write!(f, "alt"),
            Self::Opt => write!(f, "opt"),
            Self::Break => write!(f, "break"),
            Self::Par => write!(f, "par"),
            Self::Strict => write!(f, "strict"),
            Self::Loop => write!(f, "loop"),
            Self::Critical => write!(f, "critical"),
            Self::Neg => write!(f, "neg"),
            Self::Assert => write!(f, "assert"),
            Self::Ignore => write!(f, "ignore"),
            Self::Consider => write!(f, "consider"),
        }
    }
}

