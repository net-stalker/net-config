use super::network_packet_capnp::network_packet;

pub struct NetworkPacket {
    frame_time: i64,

    src_addr: String,
    dst_addr: String,

    network_packet_data: Vec<u8>,
}

impl crate::Encoder for NetworkPacket {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
    
        let mut struct_to_encode = message.init_root::<network_packet::Builder>();
        
        struct_to_encode.set_frame_time(self.frame_time);
    
        struct_to_encode.set_src_addr(&self.src_addr);
        struct_to_encode.set_dst_addr(&self.dst_addr);
        
        struct_to_encode.set_data(&self.network_packet_data);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl crate::Decoder for NetworkPacket {
    fn decode(data: Vec<u8>) -> Self {
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(), //Think about using std::io::Cursor here
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<network_packet::Reader>().unwrap();

        NetworkPacket { 
            frame_time: decoded_struct.get_frame_time(), 
            src_addr: String::from(decoded_struct.get_src_addr().unwrap()), 
            dst_addr: String::from(decoded_struct.get_dst_addr().unwrap()), 
            network_packet_data: Vec::from(decoded_struct.get_data().unwrap())
        }
    }
}