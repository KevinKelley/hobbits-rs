#![allow(unused_imports)]

extern crate tokio;
extern crate getopts;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

extern crate hobbits;

//use hobbits::{read_source};
use hobbits::tcp::server::{klk};
use hobbits::encoding::{marshal, unmarshal, Message};


fn main() {

    klk();
    marshal(Message {
        version: "123".to_string(),
        protocol: "123".to_string(),
        header:vec!(),
        body: vec!(),
    });


    // Parse command-line options:
    let mut opts = getopts::Options::new();
    opts.optopt("h", "host", "server to connect", "HOST");
    opts.optopt("p", "port", "port", "PORT");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    // Choose a format:
    let _png = match &str_arg("f", "png")[..] {
        "png" => true,
        "pdf" => false,
        x => panic!("Unknown output format: {}", x),
    };
    // Read input files:
    //let _html = read_source(str_arg("h", "examples/test.html"));

    let host = matches.opt_str("host").unwrap_or("127.0.0.1".to_string());
    let port = matches.opt_str("port").unwrap_or("12345".to_string());

    // Bind the server's socket.
    let addr = format!("{}:{}", host, port).parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
