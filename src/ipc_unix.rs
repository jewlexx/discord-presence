use crate::discord_ipc::DiscordIpc;
use crate::EventReceive;
use async_net::{unix::UnixStream, Shutdown};
use async_trait::async_trait;
use futures_lite::io::AsyncWriteExt;
use futures_lite::AsyncReadExt;
use serde_json::json;
use std::{env::var, error::Error, path::PathBuf};

// Environment keys to search for the Discord pipe
const ENV_KEYS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
#[allow(missing_docs)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
#[derive(Clone)]
pub struct DiscordIpcClient {
  /// Client ID of the IPC client.
  pub client_id: String,
  pub connected: bool,
  // Socket ref to the open socket
  pub socket: Option<UnixStream>,

  // a valid access
  pub access_token: Option<String>,
}

impl DiscordIpcClient {
  pub async fn handler<F>(&mut self, func: F)
  where
    F: Fn(EventReceive) + std::marker::Send + 'static,
  {
    let mut client = self.clone();

    tokio::spawn(async move {
      loop {
        let (_opcode, payload) = client.recv().await.unwrap();

        match serde_json::from_str::<EventReceive>(&payload) {
          Ok(e) => {
            func(e);
          }
          Err(e) => {
            println!("{:#?}", e);
          }
        }
      }
    });
  }
  /// Creates a new `DiscordIpcClient`.
  ///
  /// # Examples
  /// ```
  /// let ipc_client = DiscordIpcClient::new("<some client id>")?;
  /// ```
  pub async fn new(client_id: &str) -> Result<Self> {
    let mut client = Self {
      client_id: client_id.to_string(),
      connected: false,
      socket: None,
      access_token: None,
    };

    // connect to client
    client.connect().await?;

    Ok(client)
  }

  fn get_pipe_pattern() -> PathBuf {
    let mut path = String::new();

    for key in &ENV_KEYS {
      match var(key) {
        Ok(val) => {
          path = val;
          break;
        }
        Err(_e) => continue,
      }
    }
    PathBuf::from(path)
  }
}

#[async_trait]
impl DiscordIpc for DiscordIpcClient {
  async fn connect_ipc(&mut self) -> Result<()> {
    // iterate over the likely places to find the socket
    for i in 0..10 {
      let path = DiscordIpcClient::get_pipe_pattern().join(format!("discord-ipc-{}", i));

      match UnixStream::connect(&path).await {
        Ok(socket) => {
          println!("Found socket @ {:?}", path);
          self.socket = Some(socket);
          self.connected = true;
          return Ok(());
        }
        Err(_) => continue,
      }
    }

    Err("Couldn't connect to the Discord IPC socket".into())
  }

  async fn write(&mut self, data: &[u8]) -> Result<()> {
    let socket = self.socket.as_mut().expect("Client not connected");

    socket.write_all(data).await?;

    Ok(())
  }

  async fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
    let socket = self.socket.as_mut().unwrap();
    socket.read_exact(buffer).await?;

    Ok(())
  }

  async fn close(&mut self) -> Result<()> {
    let data = json!({});
    if self.send(data.to_string(), 2).await.is_ok() {}

    let socket = self.socket.as_mut().unwrap();

    socket.flush().await?;
    socket.shutdown(Shutdown::Both)?;

    self.connected = false;

    Ok(())
  }

  fn get_client_id(&self) -> &String {
    &self.client_id
  }
}
