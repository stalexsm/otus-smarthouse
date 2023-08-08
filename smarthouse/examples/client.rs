use tcp::client::StpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StpClient::connect("127.0.0.1:55331")?;

    let response = client.send_request("switch")?;
    println!("Response of switch command: {}", response);

    let response = client.send_request("status")?;
    println!("response of status command: {}\n", response);

    Ok(())
}
