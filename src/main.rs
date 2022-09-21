use discord_ipc::{
  models::{rpc_event::Event, commands::* },
  DiscordIpc, DiscordIpcClient, EventReceive,
};

// get all messages from the client
fn hadle_message(event: EventReceive) {
  if let EventReceive::CommandReturn(event_type) = event {
    match event_type {
      BasedCommandReturn::GetSelectedVoiceChannel { data } => {
        println!("{:#?}", data.guild_id);

        for user in data.voice_states.iter() {
          println!("{}", user.nick);
        }
      }
      BasedCommandReturn::SelectVoiceChannel { .. } => todo!(),
      _=> {
        println!("{:#?}", event_type);
      },
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
  if let Ok(mut client) = DiscordIpcClient::new(&client_id).await {
    // login to the client
    client.login(access_token).await.unwrap();

    // test join a voice channel
    // TODO: move impl to standalone class    
    client.emit(Event::get_selected_voice_channel()).await.ok();

    client.emit(Event::speaking_start_event("1022132922565804062")).await.ok();
    client.emit(Event::speaking_stop_event("1022132922565804062")).await.ok();
    
    // sub to all events to via this listener
    client.add_event_handler(hadle_message).await.ok();
  } else {
    println!("ERROR: Failed to connect to Discord IPC")
  }
}
