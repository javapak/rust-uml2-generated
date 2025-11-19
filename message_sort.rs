use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageSort {
    Synchcall,
    Asynchcall,
    Asynchsignal,
    Createmessage,
    Deletemessage,
    Reply,
}

impl std::fmt::Display for MessageSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Synchcall => write!(f, "synchCall"),
            Self::Asynchcall => write!(f, "asynchCall"),
            Self::Asynchsignal => write!(f, "asynchSignal"),
            Self::Createmessage => write!(f, "createMessage"),
            Self::Deletemessage => write!(f, "deleteMessage"),
            Self::Reply => write!(f, "reply"),
        }
    }
}

