use discord_ipc::{DiscordIpc, DiscordIpcClient, opcodes};

use std::time::Duration;
use serde_json::json;

mod models;

use crate::models::BasedEvents;

fn main()  {
  // load env vars
  dotenv::dotenv().ok();

  // access token
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new("905987126099836938").unwrap();
  client.connect().unwrap();

  std::thread::sleep(Duration::from_millis(100));

  client.login(access_token).unwrap();

  std::thread::sleep(Duration::from_millis(100));

  // 
  client.send(json!({
    "cmd": "GET_SELECTED_VOICE_CHANNEL",
    "args": {},    
  }), opcodes::OPCODES::Frame as u8).unwrap();

  loop {
    let (_opcode, payload) = client.recv().unwrap();
    let event = serde_json::from_str(&payload).unwrap();

    match event {
      BasedEvents::Login { data, .. } => {
        println!("got login event, {:#?}", data);
      },
      BasedEvents::Ready { data, .. } => {
        println!("got ready event, {:#?}", data);
      },
      _ => {

      }
      // BasedEvents::Voice { data, .. } => {
      //   println!("got ready event, {:#?}", data);
      // },
    }

  }
}
