use reqwest::Error as ReqwestError;

/// If you receive an InvalidResponse error, it means that this wrapper is outdated.
#[derive(Debug)]
pub enum InformationRequestError {
    ReqwestError(ReqwestError),
    ParseError(String),
}

#[derive(Debug)]
pub enum SetConnectionError {
    ReqwestError(ReqwestError),
    BadRequest(String),
}

/// If you receive an InvalidResponse error, it means that this wrapper is outdated.
#[derive(Debug)]
pub enum FileRequestError {
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
    ReqwestError(ReqwestError),
    Conflict(String),
    BadRequest(String),
}

#[derive(Debug)]
pub enum FileDeletionError {
    ReqwestError(ReqwestError),
    NotFound(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum JobCommandError {
    ReqwestError(ReqwestError),
    Conflict(String),
}

pub enum PrinterCommandError {
    ReqwestError(ReqwestError),
    ParseError(String),
    Conflict(String),
}

pub enum ToolCommandError {
    ReqwestError(ReqwestError),
    BadRequest(String),
    Conflict(String),
}
