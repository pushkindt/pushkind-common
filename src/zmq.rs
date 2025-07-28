pub fn send_zmq_message(msg: &[u8], zmq_address: &str) -> Result<(), zmq::Error> {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::PUSH)?;
    requester.connect(zmq_address)?;

    requester.send(msg, 0)?;

    log::info!("Sent message {} bytes to {}", msg.len(), zmq_address);

    Ok(())
}
