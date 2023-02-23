use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use simple_websockets::{Message, Responder};

use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct PullCommand<S> {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>,
    pub db_service: Arc<S>,
}

impl<S: Sender> Handler for PullCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let string_with_escapes = String::from_utf8(data).unwrap();
        // let unescaped_string = unescape(string_with_escapes.as_str()).unwrap();
        // let json_string = json!(&unescaped_string);
        // println!("string with escapes: {}", string_with_escapes);
        // println!("string without escapes: {}", unescaped_string);
        // println!("json: {}", json_string);
        println!("received from translator {:?}", string_with_escapes);

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            println!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", string_with_escapes)));
        });

        self.db_service.send(Vec::from(string_with_escapes));
    }
}