use std::{
    net::{SocketAddr, UdpSocket},
    thread,
    time::Duration,
};

use smarthouse::devices::SmartThermometer;

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);

    let receiver = args.next().unwrap_or_else(|| "127.0.0.1:43211".into());

    println!("Receiver address from args: {receiver}");

    let receiver = receiver
        .parse::<SocketAddr>()
        .expect("valid socket address expected");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr).expect("can't bind socket");
    let thermo = SmartThermometer::new("Test Thermo".to_owned());

    println!("Starting send temperature from {bind_addr} to {receiver}");
    loop {
        let temperature = thermo.get_temperature();
        let bytes = temperature.to_be_bytes();
        let send_result = socket.send_to(&bytes, receiver);
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1))
    }
}
