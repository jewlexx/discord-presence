use discord_ipc::{DiscordIpc, DiscordIpcClient, EventType, models::* };
mod models;

fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();
  let client_id = dotenv::var("CLIENT_ID").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new(&client_id).unwrap();

  // this will send the handshake to the IPC
  client.connect().unwrap();

  // this sends the login packet to the client and wait for the READY event
  client.login(access_token).unwrap();

  // send a simple event to the discord client
  let cmd = BasedCommands::GetSelectedVoiceChannel;
  client.send_cmd(cmd).ok();

  // let cmd = BasedCommands::SelectVoiceChannel { id: 123 };
  // client.send_cmd(cmd);

  loop {
    let (_opcode, payload) = client.recv().unwrap();
    println!("{}", payload);

    let event = serde_json::from_str::<EventType>(&payload);

    // handle events
    if let Ok(event) = event {

      if let EventType::Command(data) = event {
        match data {
          BasedCommands::GetSelectedVoiceChannel => {
            println!("got voice select event, {:#?}", data);
          }
          _ => {
            println!("no handled");
          }
        }
      } else if let EventType::Event(data) = event {
        match data {
          BasedEvents::Login { data, .. } => {
            println!("got login event, {:#?}", data);
          }
          BasedEvents::Ready { data, .. } => {
            println!("got ready event, {:#?}", data);
          }
          BasedEvents::Error { data, .. } => {
            println!("got err event, {:#?}", data);
          }
          // _ => {
          //   println!("no handled");
          // }
        }
      }
    } else if let Err(err) = event {
      println!("{}", err);
    }
  }
}
