mod transport_layer;
mod application_layer;
mod service;

use transport_layer::tcp::std::{tcp_client, tcp_server};

fn main() {
    service::run_client_server(
        tcp_server,
        tcp_client
    ).unwrap();
}
