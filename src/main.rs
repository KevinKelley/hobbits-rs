
extern crate tokio;
extern crate getopts;

use tokio::prelude::{Future,Stream,AsyncRead,Write};
use tokio::net::TcpListener;

extern crate hobbits;

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

            // Split up the reading and writing parts of the socket.
            let (reader, mut writer) = sock.split();

            tokio::io::read_to_end(reader, vec!())
                .and_then(move |(_, buf)| {
                    println!("AAAA received {} bytes: '{}'", buf.len(), String::from_utf8_lossy(&buf));
                    let rslt = unmarshal(&buf);
                    match rslt {
                        Ok(msg) => {
                            println!("AAAA: {}", msg);
                            if msg.protocol == "PING" {
                                let pong_msg = Envelope {
                                    protocol: "PONG".to_string(),
                                    version: "0.1".to_string(),
                                    header: msg.header,
                                    body: msg.body
                                };
                                writer.write(&marshal(&pong_msg).unwrap().to_owned()).expect("write (or marshal failed!)");
                            }
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

    // Start the Tokio runtime
    tokio::run(server);
}
