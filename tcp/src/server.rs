use super::protocol;
use crate::errors::{BindResult, ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

/// Represent STP server, that can accept incoming connections.
pub struct TcpServer {
    tcp: TcpListener,
}

impl TcpServer {
    /// Binds server to specefied socket.
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<TcpConnection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Ok(TcpConnection::new(s)),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct TcpConnection {
    stream: TcpStream,
}

impl TcpConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Send response to client
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        protocol::send_string(response, &mut self.stream)
    }

    /// Receive requests from client
    pub fn recv_request(&mut self) -> RecvResult {
        protocol::recv_string(&mut self.stream)
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
