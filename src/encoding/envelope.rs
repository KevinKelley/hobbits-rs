
extern crate hex;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Envelope represents a parsed Hobbits message.
/// See examples of unparsed and parsed messages here: https://github.com/deltap2p/hobbits/blob/master/specs/protocol.md
#[derive(Clone, Hash, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Envelope {
    pub version: String,
    pub protocol: String,
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}

impl Envelope {

    pub fn new(proto: &str, hdr: &[u8], bdy: &[u8]) -> Envelope {
        return Envelope {
            version: "0.2".to_string(),
            protocol: proto.to_string(),
            header: hdr.to_vec(),
            body: bdy.to_vec()
        }
    }

}
impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EWP {} {} {} {}\n0x{}\n0x{}",
            self.version, self.protocol, self.header.len(), self.body.len(), hex::encode(&self.header), hex::encode(&self.body))
	}
}
