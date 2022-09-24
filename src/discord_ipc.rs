use crate::models::events::BasedEvent;
use crate::opcodes::OPCODES;
use crate::pack_unpack::{pack, unpack};
use async_trait::async_trait;
use serde_json::json;
use std::error::Error;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// TODO: probably should move things we dont want the consumer to use
/// into the struct so they dont do anything bad
///
/// A client that connects to and communicates with the Discord IPC.
///
/// Implemented via the [`DiscordIpcClient`](struct@crate::DiscordIpcClient) struct.
#[async_trait]
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
    async fn connect(&mut self) -> Result<()> {
        println!("Connecting to client...");

        self.connect_ipc().await?;
        self.send_handshake().await?;

        let (_opcode, payload) = self.recv().await.unwrap();

        // spooky line is not working
        let payload = serde_json::from_str(&payload)?;
        match payload {
            BasedEvent::Ready { .. } => {
                println!("Connected to discord and got ready event!");
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
    async fn reconnect(&mut self) -> Result<()> {
        self.close().await?;
        self.connect_ipc().await?;
        self.send_handshake().await?;

        Ok(())
    }

    #[doc(hidden)]
    fn get_client_id(&self) -> &String;

    #[doc(hidden)]
    async fn connect_ipc(&mut self) -> Result<()>;

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
    async fn send_handshake(&mut self) -> Result<()> {
        self.send(
            json!({
              "v": 1,
              "client_id": self.get_client_id()
            })
            .to_string(),
            OPCODES::Handshake as u8,
        )
        .await?;

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
    async fn login(&mut self, access_token: String) -> Result<()> {
        let nonce = Uuid::new_v4().to_string();

        // TODO: move this to a struct and call send_cmd
        self.send(
            json!({
              "cmd": "AUTHENTICATE",
              "args": {
                "access_token": access_token
              },
              "nonce": nonce
            })
            .to_string(),
            OPCODES::Frame as u8,
        )
        .await?;

        self.recv().await?;

        Ok(())
    }

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
    async fn send(&mut self, data: String, opcode: u8) -> Result<()> {
        let header = pack(opcode.into(), data.len() as u32)?;

        self.write(&header).await?;
        self.write(data.as_bytes()).await?;

        Ok(())
    }

    /// send a json string payload to the socket
    async fn emit(&mut self, payload: String) -> Result<()> {
        self.send(payload, OPCODES::Frame as u8).await.unwrap();
        Ok(())
    }

    #[doc(hidden)]
    async fn write(&mut self, data: &[u8]) -> Result<()>;

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
    async fn recv(&mut self) -> Result<(u32, String)> {
        let mut header = [0; 8];

        self.read(&mut header).await?;
        let (op, length) = unpack(header.to_vec())?;

        let mut data = vec![0u8; length as usize];
        self.read(&mut data).await?;

        let response = String::from_utf8(data.to_vec())?;

        Ok((op, response))
    }

    #[doc(hidden)]
    async fn read(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// Closes the Discord IPC connection. Implementation is dependent on platform.
    async fn close(&mut self) -> Result<()>;

    async fn start() {}
}
