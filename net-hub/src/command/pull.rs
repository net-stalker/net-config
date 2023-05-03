use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use log::debug;
use simple_websockets::{Message, Responder};

use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct PullCommand {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>
}

impl Handler for PullCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let string_with_escapes = String::from_utf8(data).unwrap();
        // let unescaped_string = unescape(string_with_escapes.as_str()).unwrap();
        // let json_string = json!(&unescaped_string);
        // debug!("string with escapes: {}", string_with_escapes);
        // debug!("string without escapes: {}", unescaped_string);
        // debug!("json: {}", json_string);
        debug!("received from nobody {:?}", string_with_escapes);

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            debug!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", string_with_escapes)));
        });
    }
}