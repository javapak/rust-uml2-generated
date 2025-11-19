use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorKind {
    Assembly,
    Delegation,
}

impl std::fmt::Display for ConnectorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assembly => write!(f, "assembly"),
            Self::Delegation => write!(f, "delegation"),
        }
    }
}

