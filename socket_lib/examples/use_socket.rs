use std::io;

use socket_lib::{Command, SmartSocketClient, SmartSocketError};

fn main() {
    let mut client = match SmartSocketClient::new("127.0.0.1:43212") {
        Ok(client) => client,
        Err(SmartSocketError::Io(e)) => {
            println!("{}", e);
            return;
        }
    };

    loop {
        show_menu();
        let input = read_input();

        let response = match input {
            Some(command) => client.run_command(command).unwrap(),
            None => {
                println!("Bye...");
                break;
            }
        };

        println!("------------------");
        println!("Response: {response}");
    }
}

fn show_menu() {
    println!();
    println!("------------------");
    println!("Select action:");
    println!("1) turn off");
    println!("2) turn on");
    println!("3) is enabled");
    println!("4) power");
    println!("_) exit");
}

fn read_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Command::TurnOff,
        "2" => Command::TurnOn,
        "3" => Command::IsEnabled,
        "4" => Command::GetPower,
        _ => return None,
    };

    Some(cmd)
}
