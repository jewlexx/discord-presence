use crate::{
  models::BasedEvents,
  opcodes::OPCODES,
  pack_unpack::{pack, unpack},
};
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// const CLIENT_ID: &str = "905987126099836938";

/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
pub trait DiscordIpc {
  /// Connects the client to the Discord IPC.
  ///
  /// This method attempts to first establish a connection,
  /// and then sends a handshake.
  ///
  /// # Errors
  ///
  /// Returns an `Err` variant if the client
  /// fails to connect to the socket, or if it fails to
  /// send a handshake.
  ///
  /// # Examples
  /// ```
  /// let mut client = discord_ipc::new_client("<some client id>")?;
  /// client.connect()?;
  /// ```
  fn connect(&mut self) -> Result<()> {
    println!("Connecting to client...");

    self.connect_ipc()?;
    self.send_handshake()?;

    let (_opcode, payload) = self.recv().unwrap();

    // spooky line is not working
    let payload = serde_json::from_str(&payload)?;
    match payload {
      BasedEvents::Ready { .. } => {
        println!("Connected to dxiscord and got ready event!");
      }
      _ => {
        println!("Could not connect to discord...");
      }
    }

    Ok(())
  }

  /// Reconnects to the Discord IPC.
  ///
  /// This method closes the client's active connection,
  /// then re-connects it and re-sends a handshake.
  ///
  /// # Errors
  ///
  /// Returns an `Err` variant if the client
  /// failed to connect to the socket, or if it failed to
  /// send a handshake.
  ///
  /// # Examples
  /// ```
  /// let mut client = discord_ipc::new_client("<some client id>")?;
  /// client.connect()?;
  ///
  /// client.close()?;
  /// client.reconnect()?;
  /// ```
  fn reconnect(&mut self) -> Result<()> {
    self.close()?;
    self.connect_ipc()?;
    self.send_handshake()?;

    Ok(())
  }

  #[doc(hidden)]
  fn get_client_id(&self) -> &String;

  #[doc(hidden)]
  fn connect_ipc(&mut self) -> Result<()>;

  /// Handshakes the Discord IPC.
  ///
  /// This method sends the handshake signal to the IPC.
  /// It is usually not called manually, as it is automatically
  /// called by [`connect`] and/or [`reconnect`].
  ///
  /// [`connect`]: #method.connect
  /// [`reconnect`]: #method.reconnect
  ///
  /// # Errors
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  fn send_handshake(&mut self) -> Result<()> {
    self.send(
      json!({
          "v": 1,
          "client_id": self.get_client_id()
      }),
      OPCODES::Handshake as u8,
    )?;

    // // TODO: Return an Err if the handshake is rejected
    // NOTE: this prolly shouldnt be done here as we dont want to consume messages here
    // self.recv()?;

    Ok(())
  }

  /// Send auth
  ///
  /// This method sends the auth token to the IPC.
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  fn login(&mut self, access_token: String) -> Result<()> {
    let nonce = Uuid::new_v4().to_string();

    self.send(
      json!({
        "cmd": "AUTHENTICATE",
        "args": {
          "access_token": access_token
        },
        "nonce": nonce
      }),
      OPCODES::Frame as u8,
    )?;

    self.recv()?;

    Ok(())
  }
  /// Send auth
  ///
  /// This method sends the auth token to the IPC.
  ///
  /// Returns an `Err` variant if sending the handshake failed.
  // fn auth(&mut self) -> Result<()> {
  //   let nonce = Uuid::new_v4().to_string();
  //   self.send(
  //     json!({
  //       "cmd": "AUTHORIZE",
  //       "args": {
  //         "client_id": CLIENT_ID,
  //         "scopes": ["rpc"]
  //       },
  //       "nonce": nonce
  //     }),
  //     OPCODES::Frame as u8,
  //   )?;

  //   Ok(())

  // }

  /// Sends JSON data to the Discord IPC.
  ///
  /// This method takes data (`serde_json::Value`) and
  /// an opcode as its parameters.
  ///
  /// # Errors
  /// Returns an `Err` variant if writing to the socket failed
  ///
  /// # Examples
  /// ```
  /// let payload = serde_json::json!({ "field": "value" });
  /// client.send(payload, 0)?;
  /// ```
  fn send(&mut self, data: Value, opcode: u8) -> Result<()> {
    let data_string = data.to_string();
    let header = pack(opcode.into(), data_string.len() as u32)?;

    self.write(&header)?;
    self.write(data_string.as_bytes())?;

    Ok(())
  }

  #[doc(hidden)]
  fn write(&mut self, data: &[u8]) -> Result<()>;

  /// Receives an opcode and JSON data from the Discord IPC.
  ///
  /// This method returns any data received from the IPC.
  /// It returns a tuple containing the opcode, and the JSON data.
  ///
  /// # Errors
  /// Returns an `Err` variant if reading the socket was
  /// unsuccessful.
  ///
  /// # Examples
  /// ```
  /// client.connect_ipc()?;
  /// client.send_handshake()?;
  ///
  /// println!("{:?}", client.recv()?);
  /// ```
  fn recv(&mut self) -> Result<(u32, String)> {
    let mut header = [0; 8];

    self.read(&mut header)?;
    let (op, length) = unpack(header.to_vec())?;

    let mut data = vec![0u8; length as usize];
    self.read(&mut data)?;

    let response = String::from_utf8(data.to_vec())?;

    Ok((op, response))
  }

  #[doc(hidden)]
  fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

  /// Closes the Discord IPC connection. Implementation is dependent on platform.
  fn close(&mut self) -> Result<()>;
}
