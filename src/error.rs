// Error types
use std::error::Error as ErrorTrait;
use reqwest::Error as ReqwestError;
use csv::Error as CSVError;
// Dependencies
use std::{fmt::Display, convert::From};


#[derive(Debug)]
pub enum Error {
    // Reqwest errors
    ReqwestError(ReqwestError),
    // CSV errors
    CSVError(CSVError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Reqwest errors
            Error::ReqwestError(e) => write!(f, "Reqwest Error: {}", e.to_string()),
            // CSV errors
            Error::CSVError(e) => write!(f, "CSV Error: {}", e.to_string()),
        }
    }
}

impl ErrorTrait for Error {}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<CSVError> for Error {
    fn from(value: CSVError) -> Self {
        Error::CSVError(value)
    }
}