use std::io::Result;
use std::marker::Sized;
use std::fmt::Debug;
use models::{Payload, OpCode};

pub trait Connection
    where Self: Sized
{
    fn connect() -> Result<Self>;

    fn send<T>(&mut self, opcode: OpCode, payload: T) -> Result<()>
        where T: Payload + Debug;

    fn recv(&mut self) -> Result<Vec<u8>>;
}
