# discord-ipc-rust

Copied from [sardonicism-04/discord-rich-presence](https://github.com/sardonicism-04/discord-rich-presence)

### Why/Goals?

- Login with `access_token`
- Send RPC commands
- Receive events/commands

### Example

Simple demo of how to use this

```rust
use discord_ipc::{
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
      BasedCommandReturn::SelectVoiceChannel { .. } => todo!(),
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

  // test join a voice channel
  client
    .emit(Command::get_selected_voice_channel())
    .await
    .ok();

  client
    .emit(Event::speaking_start_event("1022132922565804062"))
    .await
    .ok();

  client
    .emit(Event::speaking_stop_event("1022132922565804062"))
    .await
    .ok();

  // sub to all events to via this listener
  client.handler(handle_message).await.ok();
}
```

### Setup

Make sure to add an `.env` file with a valid access token and client id.

```
ACCESS_TOKEN="dank_meme"
CLIENT_ID="42069420"
```

### Run locally

`make` or `carco run` to run the main file

### See also

https://github.com/sardonicism-04/discord-rich-presence
https://gitlab.com/valeth/discord-rpc-client.rs
https://github.com/ldesgoui/discord_game_sdk
https://github.com/jewlexx/discord-presence
https://discord.com/developers/docs/topics/rpc
