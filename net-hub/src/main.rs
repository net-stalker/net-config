use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self},
};

use simple_websockets::{Event, Message, Responder};

use net_commons::config::{ConfigManager, ConfigSpec, FileLoader, FileLoaderSpec};

fn main() {
    let config = ConfigManager { file_loader: Box::new(FileLoader) as Box<dyn FileLoaderSpec> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::DEALER).unwrap();
    socket.bind(&config.dealer.endpoint)
        .expect("failed bind server");

    let event_hub = simple_websockets::launch(9091)
        .expect("failed to listen on port 9001");
    let clients: Arc<RwLock<HashMap<u64, Responder>>> = Arc::new(RwLock::new(HashMap::new()));
    let clients_inner = clients.clone();

    let ws_thread_handle = thread::spawn(move || {
        loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    println!("A client connected with id #{}", client_id);
                    clients_inner
                        .write()
                        .unwrap()
                        .insert(client_id, responder.clone());
                }
                Event::Disconnect(client_id) => {
                    println!("Client #{} disconnected.", client_id);
                    clients_inner.write().unwrap().remove(&client_id);
                }
                Event::Message(client_id, message) => {
                    println!(
                        "Received a message from client #{}: {:?}",
                        client_id, message
                    );
                    // let responder = clients.get(&client_id).unwrap();
                    // let responder = clients.write().unwrap().get(&client_id).unwrap();
                    // responder.send(message);
                }
            }
        }
    });

    let dealer_thread_handle = thread::spawn(move || {
        let mut items = [socket.as_poll_item(zmq::POLLIN)];

        loop {
            let rc = zmq::poll(&mut items, -1).unwrap();
            if rc == -1 {
                break;
            }
            if items[0].is_readable() {
                let msg = socket
                    .recv_bytes(0)
                    .expect("monitor manager failed receiving response");
                println!("{:?}", msg);

                let magic_num = &msg[..4];
                if 3569595041_u32.to_be_bytes() == magic_num {
                    println!("Global header will be skipped");
                    continue;
                }

                clients.read().unwrap().iter().for_each(|endpoint| {
                    println!("Connections: {:?}", endpoint);
                    let responder = endpoint.1;
                    responder.send(Message::Text(format!("{:?}", &msg)));
                });
            }
        }
    });

    ws_thread_handle.join().unwrap();
    dealer_thread_handle.join().unwrap();
}
