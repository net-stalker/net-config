use net_control::server::cli_server;
use net_control::server::handlers::legasy_server_handler::LegasyServerHandler;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

//TODO: get rid of a strange syntax
    let server = cli_server::CLIServer::builder(DefaultServerHandler)
        .build();

    server.start_server();
}