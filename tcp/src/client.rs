use super::protocol;
use crate::errors::{ConnectResult, RequestResult};

use std::net::{TcpStream, ToSocketAddrs};

/// Represent client-side connection for STP
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// Try to connect to specified address and perform handshake.
    pub fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Ok(Self { stream })
    }

    /// Send request to connected STP server.
    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        protocol::send_string(req, &mut self.stream)?;
        let response = protocol::recv_string(&mut self.stream)?;

        Ok(response)
    }
}
