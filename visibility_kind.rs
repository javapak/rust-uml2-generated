use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisibilityKind {
    Public,
    Private,
    Protected,
    Package,
}

impl std::fmt::Display for VisibilityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Protected => write!(f, "protected"),
            Self::Package => write!(f, "package"),
        }
    }
}

