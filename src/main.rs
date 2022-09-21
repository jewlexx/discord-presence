use discord_ipc::{models::{BasedCommands::*, BasedCommandsReturn}, DiscordIpc, DiscordIpcClient, EventReceieve};

// get all messages from the client
fn hadle_message(event: EventReceieve) { 
  if let EventReceieve::CommandReturn(event_type) = event {
    match event_type {
      BasedCommandsReturn::GetSelectedVoiceChannel { data } => {
        println!("{:#?}", data.voice_states[0].nick);
      },
      BasedCommandsReturn::SelectVoiceChannel { .. } => todo!(),
    }
  } else if let EventReceieve::Event(event_type) = event {
    println!("Evt {:#?}", event_type);
  }
}

fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token from env
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();
  
  // client id from env
  let client_id = dotenv::var("CLIENT_ID").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new(&client_id).unwrap();

  // login to the client
  client.login(access_token).unwrap();

  // send a simple event to the discord client
  client.send_cmd(GetSelectedVoiceChannel).ok();

  // sub to all events to via this listener
  client.add_event_handler(hadle_message).unwrap();
}
