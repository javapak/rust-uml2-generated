use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallConcurrencyKind {
    Sequential,
    Guarded,
    Concurrent,
}

impl std::fmt::Display for CallConcurrencyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sequential => write!(f, "sequential"),
            Self::Guarded => write!(f, "guarded"),
            Self::Concurrent => write!(f, "concurrent"),
        }
    }
}

