use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ServerError {
    InitError(String),
    ParseError(String),
    HandlerError(String)
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(s) => {
                write!(f, "Parse error: {s}")
            }
            Self::HandlerError(s) => {
                write!(f, "Handler error: {s}")
            }
            Self::InitError(s) => {
                write!(f, "Init error: {s}")
            }
        }
    }
}