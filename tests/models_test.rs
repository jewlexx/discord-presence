use discord_ipc_rust::{DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
  let mut client = DiscordIpcClient::new("771124766517755954")?;

  println!("Connecting to client...");
  client.connect()?;

  println!("Get data from socket...");

  loop {
    let res = client.recv()?;
    println!("Result{:?}", res);
  }

  // wait 5 seconds and exit
  // std::thread::sleep(std::time::Duration::from_secs(5));

  client.close()?;
  Ok(())
}
