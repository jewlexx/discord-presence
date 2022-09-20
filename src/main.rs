use discord_ipc::{opcodes, DiscordIpc, DiscordIpcClient, EventType, models::* };

use serde_json::json;
use std::time::Duration;
mod models;

fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new("905987126099836938").unwrap();

  // this will send the handshake
  client.connect().unwrap();

  // this sends the login packet
  client.login(access_token).unwrap();

  client
    .send(
      json!({
        "cmd": "GET_SELECTED_VOICE_CHANNEL",
        "args": {},
        "nonce": "limga"
      }),
      opcodes::OPCODES::Frame as u8,
    )
    .unwrap();

  loop {
    let (_opcode, payload) = client.recv().unwrap();
    // println!("{}", payload);

    let event = serde_json::from_str::<EventType>(&payload);

    // handle events
    if let Ok(event) = event {

      if let EventType::Command(data) = event {
        match data {
          BasedCommands::GetSelectedVoiceChannel { data, .. } => {
            println!("got voice select event, {:#?}", data);
          }
          // _ => {
          //   println!("no handled");
          // }
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
