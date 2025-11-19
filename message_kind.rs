use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageKind {
    Complete,
    Lost,
    Found,
    Unknown,
}

impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "complete"),
            Self::Lost => write!(f, "lost"),
            Self::Found => write!(f, "found"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

