#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate tokio;
extern crate getopts;

use tokio::prelude::{Future,Stream,AsyncRead,Write};
use tokio::io::copy;
use tokio::net::TcpListener;

extern crate hobbits;

use hobbits::encoding::{marshal, unmarshal, Envelope};


//use hobbits::tcp::server::*;
fn process_envelope(env: &Envelope) {

}

fn create_server(host: &str, port: u16) -> () {

}


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
    //use std::env;
    use std::io::{self, Read, Write};
    use std::net::SocketAddr;
    use std::thread;

    use futures::sync::mpsc;
    use futures::{Sink, Future, Stream};
    use tokio_core::reactor::Core;
    use tokio_core::net::TcpListener;

    use tokio::prelude::{AsyncRead};


    use hobbits::encoding::{marshal, unmarshal, Envelope};
    use hobbits::tcp;


    #[test]
    fn peer_to_peer() {

        // Create the event loop and initiate the connection to the remote server
        let mut core = Core::new().unwrap();

        // let handle_a = core.handle();
        // let handle_b = core.handle();

        let addr_a : SocketAddr = format!("{}:{}", "127.0.0.1", 0xAAAA).parse().unwrap();
        let addr_b : SocketAddr = format!("{}:{}", "127.0.0.1", 0xBBBB).parse().unwrap();


//        let a = tcp::connect(&addr_a, &handle_a, Box::new());

        let listener_a = TcpListener::bind(&addr_a, &core.handle()).expect("unable to bind TCP listener A");
        let addr_a = listener_a.local_addr().unwrap();
        println!("listening for connections on {}", addr_a);
        let listener_b = TcpListener::bind(&addr_b, &core.handle()).expect("unable to bind TCP listener A");
        let addr_b = listener_b.local_addr().unwrap();
        println!("listening for connections on {}", addr_b);


        let client_a = listener_a.incoming();
        let welcome_a = client_a.and_then(|(sock, _peer_addr)| {
            //tokio_io::io::write_all(sock, b"Hello!\n")

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
                                let rslt = writer.write(&marshal(&pong_msg).unwrap().to_owned()).expect("write (or marshal failed!)");
                                // match rslt {
                                //     Ok(_) => {}
                                //     Err(e) => {}
                                // }
                            }
                        }
                        Err(e) => {
                            println!("ERROR: {:?}", e);
                        }
                    }
                    Ok(())
                })
                // .map(|_| ())
                // .map_err(|e| println!("socket error = {:?}", e))

        });
        let server_a = welcome_a.for_each(|()| { //(_socket, _welcome)| {
            Ok(())
        });

        //core.run(server_a).unwrap();

    }

}
