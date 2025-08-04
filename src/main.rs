mod fourth;
mod fifth;
mod sixth;
mod seventh;

use fourth::tcp::{tcp_server, tcp_client};
use std::thread;


fn main() {
    let client_thread = thread::spawn( || {
        tcp_client().unwrap();
    });
    let server_thread = thread::spawn( || {
        tcp_server().unwrap();
    });

    
    client_thread.join().unwrap();
    server_thread.join().unwrap();
}
