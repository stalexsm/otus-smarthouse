use std::error::Error;
use tcp::server::{TcpConnection, TcpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpServer::bind("127.0.0.1:55331")?;
    for connection in server.incoming() {
        process_connection(connection?)?
    }
    Ok(())
}

fn process_connection(mut conn: TcpConnection) -> Result<(), Box<dyn Error>> {
    let req = conn.recv_request()?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client")?;

    Ok(())
}
