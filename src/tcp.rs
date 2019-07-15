
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;


use std::io::{self, Read, Write};
use std::net::{SocketAddr, Shutdown};

use bytes::{BufMut, BytesMut};
use futures::prelude::*;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder};

pub fn connect(addr: &SocketAddr,
               handle: &Handle,
               stdin: Box<Stream<Item = Vec<u8>, Error = io::Error>>)
    -> Box<Stream<Item = BytesMut, Error = io::Error>>
{
    let tcp = TcpStream::connect(addr, handle);
    let handle = handle.clone();

    // After the TCP connection has been established, we set up our client
    // to start forwarding data.
    //
    // First we use the `Io::framed` method with a simple implementation of
    // a `Codec` (listed below) that just ships bytes around. We then split
    // that in two to work with the stream and sink separately.
    //
    // Half of the work we're going to do is to take all data we receive on
    // `stdin` and send that along the TCP stream (`sink`). The second half
    // is to take all the data we receive (`stream`) and then write that to
    // stdout. We'll be passing this handle back out from this method.
    //
    // You'll also note that we *spawn* the work to read stdin and write it
    // to the TCP stream. This is done to ensure that happens concurrently
    // with us reading data from the stream.
    Box::new(tcp.map(move |stream| {
        let stream = CloseWithShutdown(stream);
        let (sink, stream) = stream.framed(Bytes).split();
        let copy_stdin = stdin.forward(sink)
            .then(|result| {
                if let Err(e) = result {
                    panic!("failed to write to socket: {}", e)
                }
                Ok(())
            });
        handle.spawn(copy_stdin);
        stream
    }).flatten_stream())
}

/// A small adapter to layer over our TCP stream which uses the `shutdown`
/// syscall when the writer side is shut down. This'll allow us to correctly
/// inform the remote end that we're done writing.
struct CloseWithShutdown(TcpStream);

impl Read for CloseWithShutdown {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsyncRead for CloseWithShutdown {}

impl Write for CloseWithShutdown {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl AsyncWrite for CloseWithShutdown {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.0.shutdown(Shutdown::Write)?;
        Ok(().into())
    }
}

/// A simple `Codec` implementation that just ships bytes around.
///
/// This type is used for "framing" a TCP stream of bytes but it's really
/// just a convenient method for us to work with streams/sinks for now.
/// This'll just take any data read and interpret it as a "frame" and
/// conversely just shove data into the output location without looking at
/// it.
struct Bytes;

impl Decoder for Bytes {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        if buf.len() > 0 {
            let len = buf.len();
            Ok(Some(buf.split_to(len)))
        } else {
            Ok(None)
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        self.decode(buf)
    }
}

impl Encoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.put(&data[..]);
        Ok(())
    }
}
