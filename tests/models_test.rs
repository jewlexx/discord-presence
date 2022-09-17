use discord_ipc::{DiscordIpc, DiscordIpcClient};
use std::error::Error;
// #[test]
// fn test_models() -> Result<(), Box<dyn Error>> {
//   // load env vars
//   dotenv::dotenv().ok();

//   // connect to discord client with overlayed id
//   let mut client = DiscordIpcClient::new("905987126099836938")?;

//   client.connect()?;

//   // currently this will read the stream then drop
//   let res = client.recv()?;
//   println!("Result: {:?}", res);

//   // get access token 
//   let access_token = dotenv::var("ACCESS_TOKEN").unwrap();
//   println!("Access token, {:?}", access_token);

//   // will send an auth request for overlayed
//   let send_res = client.send(
//     json!({
//       "cmd": "AUTHENTICATE",
//       "args": {
//         "access_token": access_token
//       },
//       "nonce": "f48f6176-4afb-4c03-b1b8-d960861f5216"
//     }),
//     OPCODES::Frame as u8,
//   );

//   println!("After send: {:?}", client.recv()?);

//   // get users current channel
//   let send_res = client.send(
//     json!({
//       "cmd": "GET_SELECTED_VOICE_CHANNEL",
//       "args": {
//       },
//       "nonce": "f48f6176-4afb-4c03-b1b8-d960861f5216"
//     }),
//     OPCODES::Frame as u8,
//   );

//   println!("Channel?, {:?}", client.recv()?);

//   // client.close()?;
//   Ok(())
// }

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
  // load env vars
  dotenv::dotenv().ok();

  // access token
  let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

  // connect to discord client with overlayed id
  let mut client = DiscordIpcClient::new("905987126099836938")?;
  client.connect()?;

  let login_result = client.login(access_token);

  match login_result {
    Ok(()) => {
      println!("Login ok!");
    }
    Err(err) => {
      println!("Login error! {:?}", err);
    }
  };

  println!("Test?, {:?}", client.recv()?);

  Ok(())
}