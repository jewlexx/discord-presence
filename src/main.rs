use discord_ipc::{DiscordIpc, DiscordIpcClient, EventType, models::BasedCommands::* };

// get all messages from the client
fn hadle_message(event_type: EventType) {
  println!("Data: {:?}", event_type);
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
  client.add_event_handler(hadle_message);
}
