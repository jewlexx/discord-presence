use rpc_discord::{
  models::commands::*, models::events::BasedEvent, Command, DiscordIpc, DiscordIpcClient, Event,
  EventReceive,
};

// get all messages from the client
fn handle_message(event: EventReceive) {
  if let EventReceive::CommandReturn(event_type) = event {
    match event_type {
      BasedCommandReturn::GetSelectedVoiceChannel { data } => {
        println!("{:#?}", data.guild_id);

        for user in data.voice_states.iter() {
          println!("{}", user.nick);
        }
      }
      BasedCommandReturn::SelectVoiceChannel { data } => {
        println!("{:#?}", data.name);
      }
      _ => {
        println!("{:#?}", event_type);
      }
    }
  } else if let EventReceive::Event(event_type) = event {
    match event_type {
      BasedEvent::SpeakingStart { data } => {
        println!("{} started speaking", data.user_id);
      }
      BasedEvent::SpeakingStop { data } => {
        println!("{} stopped speaking", data.user_id);
      }
      _ => {}
    }
  }
}


const CHANNEL_ID: &str = "1019035649870934108";

#[tokio::main]
async fn main() {
  // load env vars
  dotenv::dotenv().ok();

  // access token from env
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

  // client id from env
  let client_id = dotenv::var("CLIENT_ID").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new(&client_id)
    .await
    .expect("Client failed to connect");

  // login to the client
  client.login(access_token).await.unwrap();

  client
    .emit(Command::get_selected_voice_channel())
    .await
    .ok();

  client
    .emit(Event::speaking_start_event(CHANNEL_ID))
    .await
    .ok();

  client
    .emit(Event::speaking_stop_event(CHANNEL_ID))
    .await
    .ok();

  client.handler(|e| handle_message(e)).await;
  
  println!("made it here");
}
