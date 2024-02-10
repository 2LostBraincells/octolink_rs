use reqwest::Error as ReqwestError;

/// If you receive an InvalidResponse error, it means that this wrapper is outdated.
#[derive(Debug)]
pub enum InformationRequestError {
    ServerError,
    ReqwestError(ReqwestError),
    ParseError(String),
}

#[derive(Debug)]
pub enum SetConnectionError {
    ServerError,
    ReqwestError(ReqwestError),
    BadRequest(String),
}

/// If you receive an InvalidResponse error, it means that this wrapper is outdated.
#[derive(Debug)]
pub enum FileRequestError {
    ServerError,
    ReqwestError(ReqwestError),
    ParseError(String),
    NotFound(String)
}

/// Errors that can occur when sending a file command to the server.
///
/// * `ReqwestError` occurs when the request to the server fails.
/// * `Conflict` occurs when the server responds with a `409` status code.
/// * `BadRequest` occurs when the server responds with a `400` or `500` status code, Make sure to check
///     your destination path. 
#[derive(Debug)]
pub enum FileCommandError {
    ServerError,
    ReqwestError(ReqwestError),
    Conflict(String),
    BadRequest(String),
}

#[derive(Debug)]
pub enum FileDeletionError {
    ServerError,
    ReqwestError(ReqwestError),
    NotFound(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum JobCommandError {
    ServerError,
    ReqwestError(ReqwestError),
    Conflict(String),
}

#[derive(Debug)]
pub enum DeviceStateError {
    ServerError,
    ReqwestError(ReqwestError),
    ParseError(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum ToolCommandError {
    ServerError,
    ReqwestError(ReqwestError),
    BadRequest(String),
    Conflict(String),
}
