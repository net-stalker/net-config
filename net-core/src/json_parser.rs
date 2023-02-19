use std::str::from_utf8;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use jsonpath_rust::JsonPathFinder;
use serde_json::Value;
use unescape::unescape;

pub struct JsonParser;

impl JsonParser {
    pub fn find(json_binary: Vec<u8>, path: &str) -> (Value, Vec<u8>) {
        let json = from_utf8(&json_binary).unwrap();
        let finder = JsonPathFinder::from_str(json, path).expect("path not found");

        (finder.find(), json_binary)
    }

    pub fn print(json_path_value: Value) -> String {
        format!("{}", json_path_value)
    }

    pub fn pretty(json_path_value: Value) -> String {
        format!("{:#}", json_path_value)
    }

    pub fn get_vec(json_path_value: Value) -> Vec<u8> {
        serde_json::to_vec(&json_path_value).unwrap()
    }

    pub fn get_string(mut json_path_value: Value) -> String {
        let value = json_path_value.get_mut(0).take()
            .expect("value not found");

        unescape(value.as_str().unwrap()).unwrap()
    }

    pub fn get_timestamp_with_tz(json_path_value: Value) -> DateTime<Local> {
        let format_str = "%b %d, %Y %H:%M:%S.%f %Z";
        let datetime_str = Self::get_string(json_path_value);

        Local.datetime_from_str(&*datetime_str, format_str).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};

    use crate::file::files::{Files, Reader};
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonParser::find(pcap_buffer, "$.._source.layers");
        let json = JsonParser::print(result.0);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_layer_pretty() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonParser::find(pcap_buffer, "$.._source.layers");
        let json = JsonParser::pretty(result.0);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonParser::find(pcap_buffer, "$..frame['frame.time']");
        let time = JsonParser::get_string(result.0);

        assert_eq!(time, "Sep 18, 2013 07:49:07.000000000 EEST");
    }

    #[test]
    #[should_panic]
    fn panic_if_unknown_path() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "";
        JsonParser::find(pcap_buffer, unknown_path);
    }

    #[test]
    #[should_panic]
    fn panic_if_value_not_found() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "$..frame1";
        let result = JsonParser::find(pcap_buffer, unknown_path);
        JsonParser::get_string(result.0);
    }

    #[test]
    fn expected_extract_layer_and_return_vec() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonParser::find(pcap_buffer, "$.._source.layers");
        let json = JsonParser::get_vec(result.0);

        assert_eq!(json, json_buffer);
    }

    #[test]
    fn expected_convert_frame_time_to_date_time() {
        let time = Local.datetime_from_str("Sat, 11 Feb 2023 23:40:00.000000000 EEST", "%a, %d %b %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let time = Local.datetime_from_str("Sep 18, 2013 07:49:07.000000000 EEST", "%b %d, %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let time = Local.datetime_from_str("Dec  5, 2004 21:16:24.317453000 EET", "%b %d, %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let result = JsonParser::find(pcap_buffer, "$..frame['frame.time']");
        let time = JsonParser::get_timestamp_with_tz(result.0);
        println!("{:?}", time);
        assert_eq!(time.to_string(), "2013-09-18 07:49:07 +03:00".to_string());
    }
}
