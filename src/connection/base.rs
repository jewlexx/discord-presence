use std::{
    marker::Sized,
    fmt::Debug
};

use models::{Payload, OpCode};
use error::Result;


pub trait Connection
    where Self: Sized
{
    fn connect() -> Result<Self>;

    fn send<T>(&mut self, opcode: OpCode, payload: T) -> Result<()>
        where T: Payload + Debug;

    fn recv(&mut self) -> Result<Vec<u8>>;
}
