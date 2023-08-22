use std::{
    io::{Read, Write},
    net::TcpListener,
};

use smarthouse::devices::SmartSocket;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let server_address = args.next().unwrap_or_else(|| "127.0.0.1:7890".into());

    let listener = TcpListener::bind(server_address).expect("can't bind tcp listener");
    let mut smart_socket = SmartSocket::new(String::from("Aqara Socket H1"));

    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 5] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
    }
}
