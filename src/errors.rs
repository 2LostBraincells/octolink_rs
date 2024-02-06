use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum InformationRequestError {
    ReqwestError(ReqwestError),
    InvalidResponse(String),
}

#[derive(Debug)]
pub enum SetConnectionError {
    ReqwestError(ReqwestError),
    InvalidRequest(String),
}

#[derive(Debug)]
pub enum FileRequestError {
    ReqwestError(ReqwestError),
    InvalidResponse(String),
    NoFile(String)
}
