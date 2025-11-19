use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectNodeOrderingKind {
    Unordered,
    Ordered,
    Lifo,
    Fifo,
}

impl std::fmt::Display for ObjectNodeOrderingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unordered => write!(f, "unordered"),
            Self::Ordered => write!(f, "ordered"),
            Self::Lifo => write!(f, "LIFO"),
            Self::Fifo => write!(f, "FIFO"),
        }
    }
}

