use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use log::info;
use postgres::{NoTls, Socket};
use postgres::tls::{MakeTlsConnect, TlsConnect};
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager};
use chrono::{DateTime, Local};


use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::dispatcher::CommandDispatcher;
use crate::query::insert_packet::InsertPacket;
use crate::query::query_packet::QueryPacket;
use crate::query::select_interval::{SelectInterval, self};

pub struct Timescale {
    pub thread_pool: ThreadPool,
    // for now we specify `NoTls` just for simplicity
    pub connection_pool: Pool<PostgresConnectionManager<NoTls>>
}

impl Timescale {
    pub fn new(thread_pool: ThreadPool, connection_pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self {
            thread_pool,
            connection_pool
        }
    }
}

impl NetComponent for Timescale {
    fn run(self) {
        self.thread_pool.execute(move || {
            info!("Run component");
            
            let insert_packet = InsertPacket {
                pool: Arc::new(Mutex::new(self.connection_pool.clone())),
            };
            // Test query
            let select_query = SelectInterval {
                pool: Arc::new(Mutex::new(self.connection_pool.clone()))
            };
            // you can changes timestamps to your liking
            // ============================================
            // TODO: move this code queries structure
            // let left = "2023-03-27 13:37:08.675 +0300".parse::<DateTime<Local>>().unwrap();
            // let right = "2023-03-29 15:38:10.238 +0300".parse::<DateTime<Local>>().unwrap();
            // select_query.select_packets_from_interval(left, right);
            //============================================
            let queries = Arc::new(RwLock::new(HashMap::new()));
            queries
                .write()
                .unwrap()
                .insert("insert_packet".to_string(), insert_packet);

            let packet = QueryPacket {
                pool: Arc::new(Mutex::new(self.connection_pool.clone())),
            };

            packet.subscribe();

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
        });
    }
}