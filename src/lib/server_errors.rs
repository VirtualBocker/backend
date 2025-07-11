#[derive(Debug, Clone)]
pub enum ServerError {
    InitError(String),
    HandlerError(String),
#[derive(Debug, PartialEq)]
pub enum ServerError {
    OtherError,
}
