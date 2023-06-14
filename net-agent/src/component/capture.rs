use log::info;
use net_core::transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;

use net_core::capture;
use net_core::layer::NetComponent;
use net_core::transport::connector_zeromq::ConnectorZmq;

use crate::codec::Codec;
use crate::config::Config;

pub struct Capture {
    pool: ThreadPool,
    config: Config,
}

impl Capture {
    pub fn new(pool: ThreadPool, config: Config) -> Self {
        Capture {
            pool,
            config,
        }
    }
}

impl NetComponent for Capture {
    fn run(self) {
        info!("Run component");
        let capture = pcap::Capture::from_device(self.config.capture.device_name.as_str())
            .unwrap()
            // .promisc(true)
            // .snaplen(65535)
            .buffer_size(self.config.capture.buffer_size)
            .open()
            .unwrap();
        self.pool.execute(move || {
            let client = ConnectorZmq::builder()
                .with_endpoint(self.config.agent_connector.addr)
                .with_handler(DummyCommand)
                .build()
                .connect()
                .into_inner();

            let codec = Codec::new(client);
            capture::polling::Poller::new(capture)
                .with_packet_cnt(self.config.capture.number_packages)
                .with_codec(codec)
                .poll();
        });
    }
}