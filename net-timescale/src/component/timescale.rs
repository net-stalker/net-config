use postgres::NoTls;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use net_core::transport::{
    connector_nng::{ConnectorNNG, Proto},
    polling::Poller,
    dummy_command::DummyCommand
};
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::Executor, transmitter::Transmitter
};
use crate::db_access::{
    add_traffic::add_captured_packets::AddCapturedPackets,
    query_factory::QueryFactory,
    select_by_time::select_by_time::SelectInterval
};

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
// TODO: move this to the configuration in future
pub const DISPATCHER_CONSUMER: &'static str = "inproc://nng/dispatcher_consumer";
pub const TRANSMITTER: &'static str = "inproc://nng/transmitter";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        self.thread_pool.execute(move || {
            let consumer = ConnectorNNG::pub_sub_builder()
                .with_endpoint(DISPATCHER_CONSUMER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let dispatcher = CommandDispatcher::new(consumer);
            let db_service = ConnectorNNG::pub_sub_builder()
                .with_endpoint("tcp://0.0.0.0:5556".to_string())
                .with_handler(dispatcher)
                .build_subscriber()
                .connect()
                .into_inner();
            
            let trasmitter_command = Transmitter::new(db_service.clone());
            let transmitter = ConnectorNNG::pub_sub_builder()
                .with_endpoint(TRANSMITTER.to_owned())
                .with_handler(trasmitter_command)
                .build_subscriber()
                .bind()
                .into_inner();

            let executor = Executor::new(self.connection_pool.clone());
            let result_puller = ConnectorNNG::pub_sub_builder()
                .with_endpoint(TRANSMITTER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .connect()
                .into_inner();

            let add_packets_handler = AddCapturedPackets::create_query_handler(executor.clone(),
                    result_puller.clone());
            let service_add_packets = ConnectorNNG::pub_sub_builder()
                .with_endpoint(DISPATCHER_CONSUMER.to_owned())
                .with_handler(add_packets_handler)
                .with_topic("add_packet".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();
            
            let select_by_time_interval_handler = SelectInterval::create_query_handler(executor.clone(), result_puller.clone());
            let service_select_by_time_interval = ConnectorNNG::pub_sub_builder()
                .with_endpoint(DISPATCHER_CONSUMER.to_owned())
                .with_handler(select_by_time_interval_handler)
                .with_topic("select_time".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();

            Poller::new()
                .add(service_add_packets)
                .add(service_select_by_time_interval)
                .add(transmitter)
                .add(db_service)
                .poll();
        });
    }
}