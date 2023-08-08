use super::devices::SmartSocket;

#[derive(Clone)]
pub struct SocketClient {
    socket: SmartSocket,
}

impl SocketClient {
    pub fn new(socket: SmartSocket) -> Self {
        Self { socket }
    }

    pub fn handle_cmd(&mut self, cmd: String) -> String {
        match cmd.as_str() {
            "switch" => {
                if self.socket.is_on() {
                    self.socket.switch(false);
                    String::from("The socket is switch off")
                } else {
                    self.socket.switch(true);
                    String::from("The socket is switch on")
                }
            }
            "status" => format!("{}", self.socket),
            _ => format!("Bad command {}", cmd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_switch_socket() {
        let mut client = SocketClient {
            socket: SmartSocket::new(String::from("Test socket")),
        };
        assert!(!client.socket.is_on());
        client.handle_cmd(String::from("switch"));
        assert!(client.socket.is_on());
    }
}
