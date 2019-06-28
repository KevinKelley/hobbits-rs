/// Package encoding implements message encoding and decoding for Hobbits, a Lightweight,
/// Multiclient Wire Protocol For ETH2.0 Communications.

pub mod envelope;
pub mod marshal;
pub mod unmarshal;

// publish the public interface to the encoding module
pub use envelope::Envelope;
pub use marshal::marshal;
pub use unmarshal::unmarshal;


use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EwpError {
    details: String
}

impl EwpError {
    pub fn new(msg: &str) -> EwpError {
        EwpError{details: msg.to_string()}
    }
}

impl fmt::Display for EwpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for EwpError {
    fn description(&self) -> &str {
        &self.details
    }
}

// wrap expected errors in our error type
use std::string::FromUtf8Error;
impl From<FromUtf8Error> for EwpError {
    fn from(err: FromUtf8Error) -> Self {
        EwpError::new(err.description())
    }
}

use std::num::ParseIntError;
impl From<ParseIntError> for EwpError {
    fn from(err: ParseIntError) -> Self {
        EwpError::new(err.description())
    }
}
