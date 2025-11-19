use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

