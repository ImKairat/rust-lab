use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub fn tcp_client() -> std::io::Result<()> {
    thread::sleep(Duration::from_millis(1000));

    let socket = "127.0.0.1:7878";

    let mut stream = TcpStream::connect(socket)?;
    stream.write_all(b"\nHello from client!")?;

    let mut buf = [0; 512];
    let bytes_read = stream.read(&mut buf)?;
    println!("Server response: {}", String::from_utf8_lossy(&buf[..bytes_read]));

    Ok(())
}