use std::{thread, time::Duration};

use thermo_lib::SmartThermo;

fn main() {
    let receiver_addrs = "127.0.0.1:43211";
    let thermo = SmartThermo::new(receiver_addrs).unwrap();

    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermo.get_temperature();
        println!("The temperature is {temperature}");
    }
}
