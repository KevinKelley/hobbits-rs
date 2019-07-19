extern crate bytes;
extern crate tokio_codec;

use bytes::{BufMut, BytesMut};
use tokio::codec::{Decoder, Encoder};
use tokio::prelude::*;

use crate::encoding::{Envelope, marshal, unmarshal, EwpError};

// This is where we'd keep track of any extra book-keeping information
// our transport needs to operate.
pub struct EwpCodec;

// Turns errors into std::io::Error
fn bad_data<E>(_: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Unable to decode input")
}

// Encoding is easy, we marshal our message onto the stream and send the bytes,
impl Encoder for EwpCodec {
    type Item = Envelope;
    type Error = std::io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {

        // properly we should marshal directly to 'buf'; will refactor later
        let tmp = marshal(&msg).unwrap();
        buf.reserve(tmp.len());
        buf.put(tmp);
        Ok(())
    }
}

// Decoding is easy, since we assume that message arrives a full packet; no need
// to deal with reassembing partial data in multiple steps.
impl Decoder for EwpCodec {
    type Item = Envelope;
    type Error = std::io::Error;

    // Find the next line in buf!
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {

        let msg = unmarshal(buf).map_err(std::io::Error::new(std::io::ErrorKind::InvalidData, "unparseable envelope"))?;
        // success, we got a whole Envelope.
         let bytes_used = offset + 1 + msg.header.len() + msg.body.len();
         // Cut out the used bytes from the buffer so we don't return it again.
         let _ = buf.split_to(bytes_used);
        Some(msg)
    }
}
