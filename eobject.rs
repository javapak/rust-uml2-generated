use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EObject {
}

impl EObject {
    pub fn new() -> Self {
        Self {
        }
    }

}

impl std::fmt::Display for EObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EObject(...)")
    }
}

