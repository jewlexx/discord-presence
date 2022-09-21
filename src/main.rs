// TODO: fix name?
use discord_rpc_rs::{
  models::commands::*, Command, DiscordIpc, DiscordIpcClient, Event, EventReceive,
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
      },
      _ => {
        println!("{:#?}", event_type);
      }
    }
  } else if let EventReceive::Event(event_type) = event {
    println!("Evt {:#?}", event_type);
  }
}

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

  // client
  //   .emit(Command::get_selected_voice_channel())
  //   .await
  //   .ok();

  client
    .emit(Event::speaking_start_event("1022132922565804062"))
    .await
    .ok();

  client
    .emit(Event::speaking_stop_event("1022132922565804062"))
    .await
    .ok();


  client
    .emit(Command::select_voice_channel("1022132922565804062"))
    .await
    .ok();

  // sub to all events to via this listener
  client.handler(handle_message).await.ok();

  // println!("Test blocking?");
}
