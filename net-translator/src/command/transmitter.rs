use net_core::{transport::{sockets::{Handler, Receiver, Sender}, connector_nng::{ConnectorNNG, Proto}}, topic::DB_TOPIC};

pub struct Transmitter;

impl Handler for Transmitter {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        // TODO: think about implementing ConnectorBuilderFactory
        log::info!("In transmitter");
        ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5555".to_string())
            .with_handler(crate::command::dummy::DummyCommand)
            .build_publisher()
            .connect()
            .send(data);
    }
}