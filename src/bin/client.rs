use socket2::{Socket, Domain, Type, Protocol};
use std::net::SocketAddr;
use std::io::{Read, Write};
use inquire::Text;

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();

    socket.set_read_timeout(Some(std::time::Duration::from_millis(5000))).unwrap();

    let address: SocketAddr = "127.0.0.1:1444".parse().unwrap();
    print!("Connecting to {}...\n", address);

    socket.connect(&address.into()).unwrap();

    let mut stream: std::net::TcpStream = socket.into();
    stream.write_all(b"Hello, world!\n").unwrap();

    let mut buf = [0; 1024];
    let data = stream.read(&mut buf).unwrap();
    print!("Message received: {}\n", String::from_utf8_lossy(&buf[..data]));

    loop {
        let data = Text::new("").prompt();

        match data {
            Ok(text) => {
                stream.write_all(text.as_bytes());
            }
            Err(_) => println!("Error while reading from socket!\n"),
        }

        let mut buf = [0; 1024];
        let data = stream.read(&mut buf).unwrap();
        let message = String::from_utf8_lossy(&buf[..data]);

        if message == "INVALID_MESSAGE" {
            print!("Message sended previously was invalid!")
        } else {
            print!("Message received: {}\n", message);
        }
    }
}