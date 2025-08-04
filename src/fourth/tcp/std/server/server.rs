use std::net::TcpListener;
use std::io::{Read, Write};


pub fn tcp_server() -> std::io::Result<()> {
    let socket = "127.0.0.1:7878";
    let listener = TcpListener::bind(socket)?;
    println!("\nStart listening: {socket}\n");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;

        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
        println!("Client connected from: {}\n", stream.peer_addr()?);

        stream.write_all(b"\nHello from server!\n")?;

        break;
    }

    Ok(())
}
