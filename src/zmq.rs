use serde::Serialize;

pub fn send_zmq_message<T: Serialize>(
    msg: &T,
    zmq_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::PUSH)?;
    requester.connect(zmq_address)?;

    let serialized = serde_json::to_vec(msg)?;
    requester.send(&serialized, 0)?;

    log::info!("Sent message {} bytes to {}", serialized.len(), zmq_address);

    Ok(())
}
