use std::{
    convert::From,
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json;

use super::{Command, Event, Message};
use utils;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Payload<T>
    where T: Serialize
{
    pub cmd: Command,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evt: Option<Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

impl<T> Payload<T>
    where T: Serialize
{
    pub fn with_nonce(cmd: Command, args: Option<T>, data: Option<T>, evt: Option<Event>) -> Self {
        Self { cmd, args, data, evt, nonce: Some(utils::nonce()) }
    }
}

impl<T> From<Message> for Payload<T>
    where T: Serialize + DeserializeOwned
{
    fn from(message: Message) -> Self {
        serde_json::from_str(&message.payload).unwrap()
    }
}
