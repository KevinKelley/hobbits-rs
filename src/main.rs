#![allow(unused_imports)]

extern crate tokio;
extern crate getopts;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

extern crate hobbits;

use hobbits::tcp::server::*;
use hobbits::encoding::{marshal, unmarshal, Envelope};


fn main() {

    // Parse command-line options:
    let mut opts = getopts::Options::new();
    opts.optopt("h", "host", "server to connect", "HOST");
    opts.optopt("p", "port", "port", "PORT");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let host = matches.opt_str("host").unwrap_or("127.0.0.1".to_string());
    let port = matches.opt_str("port").unwrap_or("12345".to_string());

    // Bind the server's socket.
    let addr = format!("{}:{}", host, port).parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

    println!("listening on {}", addr);
    // Pull out a stream of sockets for incoming connections
    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            println!("sock: {:?}", sock);

            // create and spawn a handler for this connection; loop immediately for next connection
            tokio::spawn({
                // Split up the reading and writing parts of the
                // socket.
                let (reader, _writer) = sock.split();

                tokio::io::read_to_end(reader, vec!())
                    .and_then(move |(_, buf)| {
                        println!("received {} bytes: '{}'", buf.len(), String::from_utf8_lossy(&buf));
                        let rslt = unmarshal(&buf);
                        match rslt {
                            Ok(msg) => {
                                println!("{}", msg);
                            }
                            Err(e) => {
                                println!("ERROR: {:?}", e);
                            }
                        }
                        Ok(())
                    })
                    .map(|_| ())
                    .map_err(|e| println!("socket error = {:?}", e))
            });
            Ok(())
        });

    // Start the Tokio runtime
    println!("starting tokio runtime");
    tokio::run(server);
}


#[cfg(test)]
mod tests {
}
