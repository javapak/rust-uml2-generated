use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpansionKind {
    Parallel,
    Iterative,
    Stream,
}

impl std::fmt::Display for ExpansionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parallel => write!(f, "parallel"),
            Self::Iterative => write!(f, "iterative"),
            Self::Stream => write!(f, "stream"),
        }
    }
}

