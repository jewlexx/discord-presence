use super::{Command, Event, Message};
use crate::utils;
use serde::{de::DeserializeOwned, Serialize};

/// The Discord client payload
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Payload<T>
where
    T: Serialize,
{
    /// The payload command
    pub cmd: Command,

    /// The payload args
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<T>,

    /// The payload data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// The payload event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evt: Option<Event>,

    /// The payload nonce
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

impl<T> Payload<T>
where
    T: Serialize,
{
    /// Create a `Payload`, by generating a nonce
    pub fn with_nonce(cmd: Command, args: Option<T>, data: Option<T>, evt: Option<Event>) -> Self {
        Self {
            cmd,
            args,
            data,
            evt,
            nonce: Some(utils::nonce()),
        }
    }
}

impl<T> From<Message> for Payload<T>
where
    T: Serialize + DeserializeOwned,
{
    fn from(message: Message) -> Self {
        serde_json::from_str(&message.payload).unwrap()
    }
}
