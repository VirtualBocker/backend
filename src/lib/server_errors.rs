#[derive(Debug, Clone)]
pub enum ServerError {
    InitError(String),
    HandlerError(String),
    OtherError,
}
