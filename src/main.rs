mod net;
mod service;

use net::transport_layer::tcp::std::{tcp_client, tcp_server};

fn main() {
    service::run_client_server(
        tcp_server,
        tcp_client,
        1000
    ).unwrap();
}
