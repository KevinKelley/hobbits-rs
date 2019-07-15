use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::thread;

use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener};
use tokio::net::{TcpStream};
use tokio::net::tcp::{ConnectFuture};

use tokio::prelude::{AsyncRead};
use tokio_codec::Framed;


use crate::encoding::{marshal, unmarshal, Envelope};
//use hobbits::tcp;
use crate::server::codec::EwpCodec;

struct Server {
    addr: SocketAddr,
    socket: ConnectFuture,
    // sink: Sync,
    // stream: TcpStream
}
impl Server {
    pub fn new(addr: &SocketAddr) -> Self {
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let socket = TcpStream::connect(&addr).and_then(|sock| {
            // split into sending and receiving sides; separate spawn for each.
            let (sink, stream) = sock.framed(EwpCodec).split();

            // // send ping
            // let sender = sink.send(Envelope{
            //     protocol: "PING".to_string(),
            //     version: "0.1".to_string(),
            //     header: vec!(),
            //     body: vec!()
            // })
            // .and_then(|_| {Ok(())})
            // .map(|_| ())
            // .map_err(|e| println!("socket error = {:?}", e));
            // // .and_then(|sink| {
            // //     let mut i = 0;
            // //     let stream = stream.take(4).map(move |msg| {
            // //         i += 1;
            // //         //println!("[a] recv: {}", String::from_utf8_lossy(&msg));
            // //         //(addr, format!("PING {}", i).into_bytes())
            // //         println!("[a] recv {}: {}", i, msg);
            // //         msg
            // //     });
            // //     sink.send_all(stream)
            // // });
            // runtime.spawn(sender);


            let receiver = stream.map(|msg| {
                println!("[b] rcv: {}", msg);
                let pong = Envelope{
                    protocol: "PONG".to_string(),
                    version: "0.1".to_string(),
                    header: vec!(),
                    body: vec!()
                };
            });
            //let b = sink.send_all(stream);
            //runtime.spawn(b);

            Ok(())
        })
        // .map(|_| ())
        // .map_err(|e| println!("socket error = {:?}", e))
        ;

        Server{
            addr: *addr,
            socket: socket,
            // sink: sink,
            // stream: stream
        }
    }
}

// let mut runtime = tokio::runtime::Runtime::new().unwrap();
// let mut core = Core::new().unwrap();
// let handle = core.handle();
// let a_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
// let b_addr: SocketAddr = "127.0.0.1:8082".parse().unwrap();

// let sock_a = TcpStream::connect(&a_addr).and_then(|sock| {
//     // split into sending and receiving sides; separate spawn for each.
//     let (sink, stream) = sock.framed(EwpCodec).split();
//
//     // send ping
//     let sender = sink.send(Envelope{
//         protocol: "PING".to_string(),
//         version: "0.1".to_string(),
//         header: vec!(),
//         body: vec!()
//     })
//     .and_then(|_| {Ok(())})
//     .map(|_| ())
//     .map_err(|e| println!("socket error = {:?}", e));
//     // .and_then(|sink| {
//     //     let mut i = 0;
//     //     let stream = stream.take(4).map(move |msg| {
//     //         i += 1;
//     //         //println!("[a] recv: {}", String::from_utf8_lossy(&msg));
//     //         //(addr, format!("PING {}", i).into_bytes())
//     //         println!("[a] recv {}: {}", i, msg);
//     //         msg
//     //     });
//     //     sink.send_all(stream)
//     // });
//     runtime.spawn(sender);
//
//
//     let receiver = stream.map(|msg| {
//         println!("[b] rcv: {}", msg);
//         let pong = Envelope{
//             protocol: "PONG".to_string(),
//             version: "0.1".to_string(),
//             header: vec!(),
//             body: vec!()
//         };
//     });
//     //let b = sink.send_all(stream);
//     //runtime.spawn(b);
//
//     Ok(())
// })
// .map(|_| ())
// .map_err(|e| println!("socket error = {:?}", e))
// ;
// let sock_b = TcpStream::connect(&b_addr).and_then(|sock| {
//     let (b_sink, b_stream) = sock.framed(EwpCodec).split();
//
//
//     //handle.spawn(b.then(|_| Ok(())));
//     //drop(core.run(a));
//
//     //Ok(())
// })
// .map(|_| ())
// .map_err(|e| println!("socket error = {:?}", e))
// ;

// // create listening servers a and b, that respond to incoming msgs
// let sock_a = TcpListener::bind(&a_addr, &handle).unwrap();
// let sock_b = TcpListener::bind(&b_addr, &handle).unwrap();
//
// // set up the listeners first
// let server_a = sock_a.incoming()
//     .for_each(|_| Ok(()))
//     .map(|_| ())
//     .map_err(|e| println!("socket error = {:?}", e));
// let server_b = sock_b.incoming()
//     .for_each(|_| Ok(()))
//     .map(|_| ())
//     .map_err(|e| println!("socket error = {:?}", e));
// runtime.spawn(server_a);
// runtime.spawn(server_b);
// runtime.spawn(sender_a);
