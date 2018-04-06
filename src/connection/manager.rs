use std::{
    thread::{self, JoinHandle},
    sync::{
        Arc,
        Mutex,
        atomic::AtomicBool,
        mpsc::{sync_channel, Receiver, SyncSender},
    },
    time,
};

use serde_json;
use serde::{Serialize, de::DeserializeOwned};

use super::Connection;
use utils;
use models::{Message, OpCode, ReadyEvent, payload::Payload};
use error::Result;

type MessageQueue = (SyncSender<Message>, Receiver<Message>);

pub struct Manager<T>
    where T: Connection + Send + Sync
{
    client_id: u64,
    send_channel: SyncSender<Message>,
    recv_channel: Receiver<Message>,
    _version: u32,
    _connected: Arc<AtomicBool>,
    _receiver: JoinHandle<()>,
    _sender: JoinHandle<()>,
    _connection: Arc<Mutex<T>>,
}

impl<T> Manager<T>
    where T: Connection + Send + Sync + 'static,
{
    pub fn with_connection(client_id: u64, connection: T) -> Result<Self> {
        let send_queue: MessageQueue = sync_channel(20);
        let recv_queue: MessageQueue = sync_channel(20);
        let conn = Arc::new(Mutex::new(connection));

        Ok(Self {
            client_id,
            send_channel: send_queue.0,
            recv_channel: recv_queue.1,
            _version: 1,
            _connected: Arc::new(AtomicBool::new(false)),
            _sender: Self::sender_loop(conn.clone(), (recv_queue.0.clone(), send_queue.1)),
            _receiver: Self::receiver_loop(conn.clone(), recv_queue.0.clone()),
            _connection: conn,
        })
    }

    pub fn send<S>(&mut self, opcode: OpCode, payload: S) -> Result<()>
        where S: Serialize + Sync + Send
    {
        let message = Message::new(opcode, payload);
        self.send_channel.send(message).unwrap();
        Ok(())
    }

    pub fn recv<S>(&mut self) -> Result<S>
        where S: DeserializeOwned + Send + Sync
    {
        let message = self.recv_channel.recv().unwrap();
        let payload = serde_json::from_str(&message.payload).unwrap();
        Ok(payload)
    }

    pub fn handshake(&mut self) -> Result<()> {
        let hs = json![{
            "client_id": self.client_id.to_string(),
            "v": 1,
            "nonce": utils::nonce()
        }];
        self.send(OpCode::Handshake, hs)?;
        let _: Payload<ReadyEvent> = self.recv()?;
        Ok(())
    }

    fn sender_loop(connection: Arc<Mutex<T>>, queue: MessageQueue) -> JoinHandle<()> {
        thread::spawn(move || {
            println!("starting sender loop...");
            loop {
                if let Ok(msg) = queue.1.recv() {
                    if let Ok(mut guard) = connection.lock() {
                        guard.send(msg).unwrap();
                        if let Ok(res) = guard.recv() {
                            queue.0.send(res).unwrap();
                        }
                    }
                };
                thread::sleep(time::Duration::from_millis(500));
            }
        })
    }

    fn receiver_loop(connection: Arc<Mutex<T>>, queue: SyncSender<Message>) -> JoinHandle<()> {
        thread::spawn(move || {
            println!("starting receiver loop...");
            loop {
                if let Ok(mut guard) = connection.lock() {
                    if let Ok(msg) = guard.recv() {
                        queue.send(msg).unwrap();
                    }
                };
                thread::sleep(time::Duration::from_millis(500));
            }
        })
    }
}
