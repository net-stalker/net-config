use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use chrono::NaiveDateTime;
use postgres::{Client, NoTls};
use serde_json::Value;

use net_core::file::files::{Files, Reader};
use net_core::json_parser::JsonParser;
use net_core::json_pcap_parser::JsonPcapParser;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use net_timescale::command::dispatcher::CommandDispatcher;
use net_timescale::query::insert_packet::InsertPacket;

fn main() {
    thread::spawn(move || {
        let connection = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();
        let insert_packet = InsertPacket { conn: Arc::new(Mutex::new(connection)) };

        let queries = Arc::new(RwLock::new(HashMap::new()));
        queries.write().unwrap().insert("insert_packet".to_string(), insert_packet);

        let command_dispatcher = CommandDispatcher { queries };

        let db_service = ConnectorNNG::builder()
            .with_endpoint("tcp://0.0.0.0:5556".to_string())
            .with_proto(Proto::Rep)
            .with_handler(command_dispatcher)
            .build()
            .bind()
            .into_inner();

        Poller::new()
            .add(db_service)
            .poll();
    }).join().unwrap();
}