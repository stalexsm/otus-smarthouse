use smarthouse::{clients::SocketClient, devices::SmartSocket};
use std::thread;
use tcp::server::{TcpConnection, TcpServer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_client = SocketClient::new(SmartSocket::new(String::from("Socket")));
    let receiver = TcpServer::bind("127.0.0.1:55331")?;
    for connection in receiver.incoming() {
        let connection = match connection {
            Ok(connection) => connection,
            Err(error) => {
                eprintln!("Can't establish connection: {}", error);
                continue;
            }
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        let socket_client = socket_client.clone();
        thread::spawn(move || {
            if handle_connection(connection, socket_client).is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
    Ok(())
}

fn handle_connection(
    mut connection: TcpConnection,
    mut socket_client: SocketClient,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(command) = connection.recv_request() {
        let response = socket_client.handle_cmd(command);
        connection.send_response(response)?;
    }
    Ok(())
}
