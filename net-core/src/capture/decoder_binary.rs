use subprocess::{Exec, Redirection};

use crate::translator::Decoder;

pub struct JsonDecoder;

impl Decoder for JsonDecoder {
    type Input = Vec<u8>;
    type Output = Vec<u8>;

    /// https://tshark.dev/capture/tshark/
    ///
    /// # Arguments
    ///
    /// * `buf`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn decode(buf: Vec<u8>) -> Vec<u8> {
        Exec::cmd("tshark")
            .arg("-V") //add output of packet tree        (Packet Details)
            // .arg("-c1") //add output of packet tree        (Packet Details)
            // .arg("-rcaptures/arp.pcap") // set the filename to read from (or '-' for stdin)
            .arg("-r") // set the filename to read from (or '-' for stdin)
            .arg("-")
            // .arg("-x") //add output of hex and ASCII dump (Packet Bytes)
            .arg("-Tjson") //pdml|ps|psml|json|jsonraw|ek|tabs|text|fields| format of text output (def: text)
            .arg("--no-duplicate-keys") // If -T json is specified, merge duplicate keys in an object into a single key with as value a json array containing all values
            .stdin(buf)
            .stdout(Redirection::Pipe)
            .capture().unwrap()
            .stdout
    }
}

#[cfg(test)]
mod tests {
    use crate::test_resources;
    use crate::file::files::{Files, Reader};

    use super::*;

    #[test]
    fn expected_decode_arp_pcap() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.pcap"));
        let json_buffer = Files::read(test_resources!("captures/arp.json"));

        let json_result = JsonDecoder::decode(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_decode_dhcp_pcap() {
        let pcap_buffer = Files::read(test_resources!("captures/dhcp.pcap"));
        let json_buffer = Files::read(test_resources!("captures/dhcp.json"));

        let json_result = JsonDecoder::decode(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }
}
