use failure::Fail;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeErrorResponse {
    pub error_message: Option<ExchagneErrorMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExchagneErrorMessage {
    code: i32,
    // name: String,
    #[serde(rename = "msg")]
    message: String,
}

impl From<ExchagneErrorMessage> for EdpError {
    fn from(msg: ExchagneErrorMessage) -> EdpError {
        EdpError::RemoteError {
            code: msg.code,
            // name: msg.name,
            message: msg.message
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Fail)]
pub enum EdpError {
    #[fail(display = "No Api Key Set")]
    NoApiKeySet,
    #[fail(display = "Error Message From Exchange, code {}, message {}", code,  message)]
    // RemoteError {code: i32, name: String, message: String},
    RemoteError {code: i32, message: String},
}