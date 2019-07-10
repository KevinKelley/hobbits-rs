extern crate bytes;
use bytes::{BufMut, BytesMut};
use tokio::codec::{Decoder, Encoder};
use tokio::prelude::*;

use crate::encoding::{Envelope, marshal, unmarshal, EwpError};

// This is where we'd keep track of any extra book-keeping information
// our transport needs to operate.
struct EwpCodec;

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
        let tmp = marshal(msg);
        buf.reserve(tmp.len());
        buf.put(tmp);
        Ok(())
    }
}

// The decoding is a little trickier, because we need to deal with messages that aren't completely
// here yet. We also need to handle *two* cases: the "normal"
// case where we're just asked to find the next envelope in a bunch of
// bytes, and the "end" case where the input has ended, and we need
// to find any remaining ones...
impl Decoder for EwpCodec {
    type Item = Envelope;
    type Error = std::io::Error;

    // Find the next line in buf!
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let retval = if let Some(offset) = buf.iter().position(|b| *b == b'\n') {
        //     // We found a newline character in this buffer...
        //     // parse envelope header line; and check if we also have enough bytes for payload
        //
            match unmarshal(buf) {
                Ok(msg) => {
                    // success, we got a whole Envelope.
                    let bytes_used = offset + 1 + msg.header.len() + msg.body.len();
                    // Cut out the used bytes from the buffer so we don't return it again.
                    let _ = buf.split_to(bytes_used);
                    Some(msg)
                },
                Err(e) => {
                    // fail, envelope didn't parse.  Maybe payload is still coming?
                    // TODO: how to recover, from a malformed Envelope?  Depending on why it failed
                    // to parse... maybe not complete yet?  Then wait.  But... if it's a bad envelope,
                    // then what?  If we can't parse the envelope header... for example... then we
                    // don't know how much data to throw away, to get to the next 'good' envelope...
                    //
                    // anyway, need to check the errors 'e' here, and handle the ones we can.
                    //
                    // otherwise, discard the buf and try to continue... could search for '\n' and
                    // back up, to see if it's a valid header for next envelope...
                    None
                }
            }
        } else {
        //     // There are no newlines in this buffer, or else not enough payload;
        //     // Tokio will make sure to call this again when we have more bytes.
            None
        };
        Ok(retval)
    }

    // Find the next envelope in buf when there will be no more data coming.
    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf) {
            Ok(Some(frame)) => {
                // There's a regular line here, so we may as well just return that.
                Ok(Some(frame))
            },
            Ok(None) => {
                // None, means we couldn't decode the remaining data.
                if buf.is_empty() {
                    // if there's no data, we're good
                    Ok(None)
                } else {
                    // but if there's remaining bytes, that we couldn't decode, signal an error
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "unparseable input remaining on stream"))
                }
            }
            Err(e) => {
                // decode failed, pass on the error.
                Err(e)
            }
        }
    }
}
