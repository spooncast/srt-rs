extern crate bytes;
extern crate srt;

extern crate futures;

extern crate tokio;

use std::net::ToSocketAddrs;
use srt::socket::{SrtSocketBuilder};

use futures::prelude::*;

use tokio::executor::current_thread;

fn main() {
    let conn = SrtSocketBuilder::new("127.0.0.1:1231".to_socket_addrs().unwrap().next().unwrap())
        .build()
        .unwrap()
        .and_then(|(sock, addr)| {
            println!("Connected to {:?}", addr);
        });

    current_thread::run(|_| {
        current_thread::spawn(conn.map_err(|e| {
            eprintln!("Error received: {:?}", e);
        }));
    });
}
