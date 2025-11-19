use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationKind {
    None,
    Shared,
    Composite,
}

impl std::fmt::Display for AggregationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Shared => write!(f, "shared"),
            Self::Composite => write!(f, "composite"),
        }
    }
}

