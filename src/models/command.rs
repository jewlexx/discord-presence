use serde::Serialize;
use uuid::Uuid;
use super::Payload;

#[derive(Debug, Default, Serialize)]
pub struct Command<T>
    where T: Serialize
{
    pub nonce: String,
    pub cmd: String,
    pub args: T,
}

impl<T> Command<T>
    where T: Serialize
{
    pub fn new<S>(cmd: S, args: T) -> Self
        where S: Into<String>
    {
        Command {
            cmd: cmd.into(),
            nonce: Uuid::new_v4().to_string(),
            args: args
        }
    }
}

impl<T> Payload for Command<T>
    where T: Serialize {}
