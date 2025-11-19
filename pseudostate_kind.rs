use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PseudostateKind {
    Initial,
    Deephistory,
    Shallowhistory,
    Join,
    Fork,
    Junction,
    Choice,
    Entrypoint,
    Exitpoint,
    Terminate,
}

impl std::fmt::Display for PseudostateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initial => write!(f, "initial"),
            Self::Deephistory => write!(f, "deepHistory"),
            Self::Shallowhistory => write!(f, "shallowHistory"),
            Self::Join => write!(f, "join"),
            Self::Fork => write!(f, "fork"),
            Self::Junction => write!(f, "junction"),
            Self::Choice => write!(f, "choice"),
            Self::Entrypoint => write!(f, "entryPoint"),
            Self::Exitpoint => write!(f, "exitPoint"),
            Self::Terminate => write!(f, "terminate"),
        }
    }
}

