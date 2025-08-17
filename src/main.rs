#![allow(dead_code)]
mod async_rust;
mod common;
mod data_types;
mod multi_processing;
mod net;

use net::transport_layer::tcp::std::{tcp_client, tcp_server};

fn main() {
    common::run_client_server(tcp_server, tcp_client, 1000).unwrap();

    let nman: Man = Man {
        name: "Kairat".to_string(),
        age: 23_u8,
    };
    println!("{:#?}", nman);
}

#[derive(Debug)]
struct Man {
    name: String,
    age: u8,
}
