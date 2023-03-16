
use std::sync::{Arc};



use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct AgentCommand<S> {
    pub translator: Arc<S>,
}

impl<S: Sender> Handler for AgentCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        debug!("received from agent {:?}", data);

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // debug!("Global header will be skipped");
        // return;
        // }

        self.translator.send(data);
    }
}