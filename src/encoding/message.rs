
// // Package encoding implements message encoding and decoding for Hobbits, a Lightweight,
// // Multiclient Wire Protocol For ETH2.0 Communications.
// //
// // By Rene Nayman
// package encoding
//
// // Message represents a parsed Hobbits message.
// // See examples of unparsed and parsed messages here: https://github.com/deltap2p/hobbits/blob/master/specs/protocol.md
// type Message struct {
// 	Version  string
// 	Protocol string
// 	Header   []byte
// 	Body     []byte
// }

pub struct Message {
    pub version: String,
    pub protocol: String,
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}
