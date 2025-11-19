use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

