// ============================================================================
// Generated Rust Code
// ============================================================================
//
// Type:           MessageSort (enum)
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
pub enum MessageSort {
    Synchcall,
    Asynchcall,
    Asynchsignal,
    Createmessage,
    Deletemessage,
    Reply,
}

impl std::fmt::Display for MessageSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Synchcall => write!(f, "synchCall"),
            Self::Asynchcall => write!(f, "asynchCall"),
            Self::Asynchsignal => write!(f, "asynchSignal"),
            Self::Createmessage => write!(f, "createMessage"),
            Self::Deletemessage => write!(f, "deleteMessage"),
            Self::Reply => write!(f, "reply"),
        }
    }
}

