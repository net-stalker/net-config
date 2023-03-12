use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct TranslatorCommand;

impl Handler for TranslatorCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        debug!("received from agent {:?}", data);
    }
}