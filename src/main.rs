#![allow(unused_imports)]

extern crate tokio;
extern crate getopts;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

extern crate hobbits;

use hobbits::tcp::server::*;
use hobbits::encoding::{marshal, unmarshal, Message};


fn main() {

    let msg = Message {
        version: "0.2".to_string(),
        protocol: "GOSSIP".to_string(),
        header:"hdr".as_bytes().to_vec(),
        body: "body".as_bytes().to_vec(),
    };
    println!("{}\n{:?}", msg, msg);

    let serialized = serde_json::to_string(&msg).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);


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

if true {return};

    let messages: Vec<Message> = vec!();

    // Pull out a stream of sockets for incoming connections
    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            // Split up the reading and writing parts of the
            // socket.
            let (reader, writer) = sock.split();

            // A future that echos the data and returns how
            // many bytes were copied...
            let bytes_copied = copy(reader, writer);

            // ... after which we'll print what happened.
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            });

            // Spawn the future as a concurrent task.
            tokio::spawn(handle_conn)
        });

    // Start the Tokio runtime
    tokio::run(server);
}


#[cfg(test)]
mod tests {
}
