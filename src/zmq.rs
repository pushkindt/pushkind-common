use serde::Serialize;
use std::{thread, time::Duration};
use tokio::sync::mpsc;
use zmq;

/// How many messages to buffer before applying backpressure.
const DEFAULT_QUEUE_CAPACITY: usize = 10_000;

/// Which socket type to create in the background thread.
#[derive(Clone, Copy, Debug)]
pub enum SocketKind {
    Pub,
    Push,
}

/// Tunables for the sender thread.
#[derive(Clone, Debug)]
pub struct ZmqSenderOptions {
    pub endpoint: String,
    pub kind: SocketKind,
    pub queue_capacity: usize,
    pub sndhwm: i32,     // high-water mark for outbound queue
    pub linger_ms: i32,  // linger on drop
    pub immediate: bool, // don't queue to not-yet-connected peers
    pub warmup_ms: u64,  // one-time sleep after connect (helps PUB)
}

impl ZmqSenderOptions {
    pub fn pub_default(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            kind: SocketKind::Pub,
            queue_capacity: DEFAULT_QUEUE_CAPACITY,
            sndhwm: 100_000,
            linger_ms: 0,
            immediate: true,
            warmup_ms: 300, // give SUB→XPUB→XSUB→PUB time to propagate
        }
    }
    pub fn push_default(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            kind: SocketKind::Push,
            queue_capacity: DEFAULT_QUEUE_CAPACITY,
            sndhwm: 100_000,
            linger_ms: 0,
            immediate: true,
            warmup_ms: 50, // PUSH doesn't need much, but a tiny settle time is fine
        }
    }
}

/// Payload variants the thread can send.
enum Envelope {
    Bytes(Vec<u8>),
    Multipart(Vec<Vec<u8>>),
}

/// Handle your routes can clone and use.
#[derive(Clone)]
pub struct ZmqSender {
    tx: mpsc::Sender<Envelope>,
}

#[derive(thiserror::Error, Debug)]
pub enum ZmqSenderError {
    #[error("serialize error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("queue is full")]
    QueueFull,
    #[error("sender thread is closed")]
    ChannelClosed,
    #[error("create ZMQ socket: {0}")]
    SocketCreate(zmq::Error),
    #[error("connect {endpoint} failed: {source}")]
    Connect {
        endpoint: String,
        source: zmq::Error,
    },
}

impl ZmqSender {
    /// Spawn a dedicated thread that owns the ZeroMQ socket.
    pub fn start(opts: ZmqSenderOptions) -> Result<Self, ZmqSenderError> {
        let (tx, mut rx) = mpsc::channel::<Envelope>(opts.queue_capacity);

        let ctx = zmq::Context::new();
        let ty = match opts.kind {
            SocketKind::Pub => zmq::PUB,
            SocketKind::Push => zmq::PUSH,
        };
        let sock = ctx.socket(ty).map_err(ZmqSenderError::SocketCreate)?;

        // Reasonable defaults
        sock.set_sndhwm(opts.sndhwm).ok();
        sock.set_linger(opts.linger_ms).ok();
        sock.set_immediate(opts.immediate).ok();

        let endpoint = opts.endpoint.clone();
        sock.connect(&endpoint)
            .map_err(|source| ZmqSenderError::Connect { endpoint, source })?;

        let kind = opts.kind;
        let warmup_ms = opts.warmup_ms;

        thread::spawn(move || {
            if warmup_ms > 0 {
                thread::sleep(Duration::from_millis(warmup_ms));
            }

            while let Some(env) = rx.blocking_recv() {
                let res = match env {
                    Envelope::Bytes(b) => sock.send(b, 0),
                    Envelope::Multipart(frames) => sock.send_multipart(frames, 0),
                };
                if let Err(e) = res {
                    // You can swap for `log::error!` if you prefer structured logging here.
                    log::error!("[ZmqSender {:?}] send error: {e}", kind);
                    // Tiny backoff prevents hot-looping on repeated failure
                    thread::sleep(Duration::from_millis(50));
                }
            }
            // Channel closed => exit; linger=0 makes teardown fast.
        });

        Ok(Self { tx })
    }

    /// Send raw bytes (awaits if the queue is full).
    pub async fn send_bytes(&self, bytes: Vec<u8>) -> Result<(), ZmqSenderError> {
        self.tx
            .send(Envelope::Bytes(bytes))
            .await
            .map_err(|_| ZmqSenderError::ChannelClosed)
    }

    /// Try to send raw bytes (fails fast if the queue is full).
    pub fn try_send_bytes(&self, bytes: Vec<u8>) -> Result<(), ZmqSenderError> {
        self.tx
            .try_send(Envelope::Bytes(bytes))
            .map_err(|e| match e {
                mpsc::error::TrySendError::Full(_) => ZmqSenderError::QueueFull,
                mpsc::error::TrySendError::Closed(_) => ZmqSenderError::ChannelClosed,
            })
    }

    /// Convenience: serialize JSON and send (single-frame).
    pub async fn send_json<T: Serialize>(&self, v: &T) -> Result<(), ZmqSenderError> {
        let bytes = serde_json::to_vec(v)?;
        self.send_bytes(bytes).await
    }

    /// PUB-friendly helper: send a topic + JSON as multipart.
    /// Works with PUSH too (PULL will just receive two frames).
    pub async fn send_topic_json<T: Serialize>(
        &self,
        topic: impl Into<Vec<u8>>,
        v: &T,
    ) -> Result<(), ZmqSenderError> {
        let payload = serde_json::to_vec(v)?;
        let frames = vec![topic.into(), payload];
        self.tx
            .send(Envelope::Multipart(frames))
            .await
            .map_err(|_| ZmqSenderError::ChannelClosed)
    }
}
