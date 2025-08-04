mod fourth;
mod fifth;
mod sixth;
mod seventh;
mod service;

use fourth::tcp::std::{tcp_client, tcp_server};

fn main() {
    service::run_client_server(
        tcp_server,
        tcp_client
    ).unwrap();
}
