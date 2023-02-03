use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use simple_websockets::{Message, Responder};
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct AgentCommand<S> {
    pub translator: Arc<S>,
}

impl<S: Sender> Handler for AgentCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let mut data = receiver.recv();
        println!("received from agent {:?}", data);

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // println!("Global header will be skipped");
        // return;
        // }

        self.translator.send(data);
    }
}