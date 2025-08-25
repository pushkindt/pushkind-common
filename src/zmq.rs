use serde::Serialize;
use std::{thread, time::Duration};

/// Serialize `msg` and send it to `zmq_address` using a ZMQ `PUB` socket.
///
/// A new socket is created for each call. Any serialization or socket errors
/// are bubbled up to the caller.
pub fn send_zmq_message<T: Serialize>(
    msg: &T,
    zmq_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::PUB)?;
    requester.connect(zmq_address)?;

    // Give the proxy/subscribers a moment to connect & propagate subscriptions
    thread::sleep(Duration::from_millis(200));

    let serialized = serde_json::to_vec(msg)?;
    requester.send(&serialized, 0)?;

    log::info!("Sent {} bytes to {}", serialized.len(), zmq_address);

    Ok(())
}
