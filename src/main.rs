use discord_ipc::{
    models::{BasedCommands::*, BasedCommandsReturn},
    DiscordIpc, DiscordIpcClient, EventReceive,
};

// get all messages from the client
fn hadle_message(event: EventReceive) {
    if let EventReceive::CommandReturn(event_type) = event {
        match event_type {
            BasedCommandsReturn::GetSelectedVoiceChannel { data } => {
                println!("{:#?}", data.guild_id);

                for user in data.voice_states.iter() {
                    println!("{}", user.nick);
                }
            }
            BasedCommandsReturn::SelectVoiceChannel { .. } => todo!(),
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

        // send a simple event to the discord client
        client.send_cmd(GetSelectedVoiceChannel).await.ok();

        // sub to all events to via this listener
        client.add_event_handler(hadle_message).await.ok();
    } else {
        println!("ERROR: Failed to connect to Discord IPC")
    }
}
