use log::info;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_translator::component::translator::Translator;
use net_translator::config::Config;

fn main() {
    init_log();
    info!("Run module");

    let config = Config::builder().build().expect("read config error");
    let pool = ThreadPool::with_name("worker".into(), 5);

    Translator::new(pool.clone(), config).run();

    pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
