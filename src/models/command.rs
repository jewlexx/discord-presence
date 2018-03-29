use serde::Serialize;

use super::Payload;
use utils::nonce;


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
            nonce: nonce(),
            args: args
        }
    }
}

impl<T> Payload for Command<T>
    where T: Serialize {}
