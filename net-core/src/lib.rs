pub mod capture;
pub mod config;
pub mod transport;
pub mod file;
pub mod json_parser;
pub mod json_pcap_parser;

#[macro_export]
macro_rules! test_resources {($fname:expr) => (
        // The environment variable CARGO_MANIFEST_DIR provide a stable base point to reference other files.
        // Here, we assume that there's a test/resources directory at the top level of the crate
        concat!(env!("CARGO_MANIFEST_DIR"), "/test/resources/", $fname)
)}