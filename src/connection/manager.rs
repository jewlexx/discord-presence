use std::{
    thread::{self, JoinHandle},
    sync::{
        Arc,
        Mutex,
        atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering},
        mpsc::{sync_channel, Receiver, SyncSender},
    },
    time,
    io::ErrorKind
};

use serde_json;
use serde::{Serialize, de::DeserializeOwned};

use super::{
    Connection as BaseConnection,
    SocketConnection,
};
use utils;
use models::{Message, OpCode};
use error::{Result, Error};

type MessageQueue = (SyncSender<Message>, Receiver<Message>);
type Connection = Arc<Mutex<Option<SocketConnection>>>;

static CONNECTED: AtomicBool = ATOMIC_BOOL_INIT;
static STARTED: AtomicBool = ATOMIC_BOOL_INIT;
static HANDSHAKED: AtomicBool = ATOMIC_BOOL_INIT;
static HANDSHAKING: AtomicBool = ATOMIC_BOOL_INIT;

pub struct Manager {
    send_channel: SyncSender<Message>,
    recv_channel: Receiver<Message>,
    _sender: JoinHandle<()>,
    _checker: JoinHandle<()>,
    _connection: Connection,
}

impl Manager {
    pub fn new(client_id: u64) -> Result<Self> {
        let send_queue: MessageQueue = sync_channel(20);
        let recv_queue: MessageQueue = sync_channel(20);
        let conn = Arc::new(Mutex::new(None));
        let send_channel = send_queue.0;
        let recv_channel = recv_queue.1;

        let manager = Self {
            send_channel,
            recv_channel,
            _sender: Self::sender_loop(conn.clone(), (recv_queue.0.clone(), send_queue.1)),
            _checker: Self::connection_checker(conn.clone(), client_id),
            _connection: conn,
        };

        Ok(manager)
    }

    pub fn start(&mut self) {
        STARTED.store(true, Ordering::SeqCst);
    }

    pub fn send<S>(&mut self, opcode: OpCode, payload: S) -> Result<()>
        where S: Serialize + Sync + Send
    {
        let message = Message::new(opcode, payload);
        self.send_channel.send(message).unwrap();
        Ok(())
    }

    pub fn recv<S>(&mut self) -> Result<(OpCode, S)>
        where S: DeserializeOwned + Send + Sync
    {
        let message = self.recv_channel.recv_timeout(time::Duration::from_secs(10))?;
        let payload = serde_json::from_str(&message.payload)?;
        Ok((message.opcode, payload))
    }

    fn connect(connection: Connection) -> Result<()> {
        if !CONNECTED.load(Ordering::SeqCst) {
            if let Ok(mut conn_lock) = connection.lock() {
                if conn_lock.is_some() {
                    if let Some(ref mut conn) = *conn_lock {
                        if let Ok(opcode) = Self::ping(conn) {
                            if opcode == OpCode::Pong {
                                CONNECTED.store(true, Ordering::SeqCst);
                                debug!("Reconnected")
                            }
                        }
                    }
                } else {
                    *conn_lock = Some(SocketConnection::connect()?);
                    CONNECTED.store(true, Ordering::SeqCst);
                    debug!("Connected")
                }
            }
        }
        Ok(())
    }

    fn disconnect(connection: Connection) {
        if let Ok(mut conn_lock) = connection.lock() {
            if let Some(ref mut conn) = *conn_lock {
                if conn.disconnect().is_ok() {
                    CONNECTED.store(false, Ordering::SeqCst);
                    HANDSHAKED.store(false, Ordering::SeqCst);
                }
            }

            *conn_lock = None;
        }
    }

    fn handshake(connection: Connection, client_id: u64) -> Result<()> {
        if CONNECTED.load(Ordering::SeqCst) && !HANDSHAKED.load(Ordering::SeqCst) && !HANDSHAKING.load(Ordering::SeqCst) {
            let hs = json![{
                "client_id": client_id.to_string(),
                "v": 1,
                "nonce": utils::nonce()
            }];

            if let Ok(mut conn_guard) = connection.lock() {
                if let Some(ref mut conn) = *conn_guard {
                    conn.send(Message::new(OpCode::Handshake, hs))?;
                    let _res = conn.recv()?;
                    HANDSHAKED.store(true, Ordering::SeqCst);
                }
            }
        }
        Ok(())
    }

    fn ping(connection: &mut SocketConnection) -> Result<OpCode> {
        let message = Message::new(OpCode::Ping, json![{}]);
        connection.send(message)?;
        let opcode = connection.recv()?.opcode;
        debug!("{:?}", opcode);
        Ok(opcode)
    }

    fn send_and_receive(connection: &Connection, queue: &MessageQueue) -> Result<()> {
        if let Ok(msg) = queue.1.recv() {
            if let Ok(mut conn_guard) = connection.lock() {
                if let Some(ref mut conn) = *conn_guard {
                    conn.send(msg)?;
                    let res = conn.recv()?;
                    queue.0.send(res).unwrap();
                }
            }
        };
        Ok(())
    }

    fn connection_checker(connection: Connection, client_id: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            debug!("Starting connection checker loop...");

            loop {
                let _ = Self::connect(connection.clone());
                match Self::handshake(connection.clone(), client_id) {
                    Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => {
                        debug!("{:?}", err);
                    },
                    Err(err) => debug!("{:?}", err),
                    Ok(_) => ()
                };

                thread::sleep(time::Duration::from_millis(500));
            }
        })
    }

    fn sender_loop(connection: Connection, queue: MessageQueue) -> JoinHandle<()> {
        thread::spawn(move || {
            debug!("Starting sender loop...");
            loop {
                if STARTED.load(Ordering::SeqCst) && CONNECTED.load(Ordering::SeqCst) && HANDSHAKED.load(Ordering::SeqCst) {
                    match Self::send_and_receive(&connection, &queue) {
                        Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(Error::IoError(_err)) => {
                            Self::disconnect(connection.clone());
                            // error!("Disconnected: {}", _err);
                        },
                        Err(why) => error!("{}", why),
                        Ok(_) => ()
                    }
                }
                thread::sleep(time::Duration::from_millis(500));
            }
        })
    }
}
