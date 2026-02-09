use std::io::{Read, Write};
use socket2::{Socket, Domain, Type, Protocol};
use std::net::{Shutdown, SocketAddr};

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
    socket.set_reuse_address(true).unwrap();

    let address: SocketAddr = "127.0.0.1:1444".parse().unwrap();
    socket.bind(&address.into()).unwrap();

    socket.listen(128).unwrap();

    let listener: std::net::TcpListener = socket.into();

    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut conn) => {
                println!("New connection!");

                let _ = conn.write_all(b"Hello, world!!");

                let mut buffer = [0; 1024];

                loop {
                    match conn.read(&mut buffer) {
                        Ok(0) => {
                            println!("Connection closed!");
                            break
                        }
                        Ok(size) => {
                            let message = String::from_utf8_lossy(&buffer[..size]);
                            println!("Message received: {}", message);

                            if message == "Hello!" {
                                conn.write_all(b"Hello!");
                            } else {
                                conn.write_all(b"INVALID_MESSAGE");
                            }
                        }
                        Err(e) => {
                            println!("Connection error: {}", e);
                        }
                    }
                }
            },
            Err(_) => todo!()
        }
    }
}
