use std::num::TryFromIntError;
use std::os::unix::io::RawFd;
use nng::{Protocol, Socket};
use nng::options::{Options, RecvFd};
use rand::{Rng, thread_rng};
use zmq::SocketType;
use crate::transport;

use crate::transport::context::{Context, ContextBuilder};
use crate::transport::sockets;
use crate::transport::sockets::{Handler, Receiver, Sender};

//TODO Connector Builder should be redesigned as Fluent API with constraints.

pub struct ConnectorNng<HANDLER> {
    endpoint: String,
    handler: Option<Box<HANDLER>>,
    socket: Socket,
}

impl<HANDLER> Receiver for ConnectorNng<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv()
            .unwrap()
            .as_slice()
            .to_vec() //note: every time data is coped from stack to the heap!
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorNng<HANDLER>
{
    fn fd(&self) -> RawFd {
        self.socket.get_opt::<RecvFd>().unwrap()
    }

    fn fd_as_usize(&self) -> Result<usize, TryFromIntError> {
        usize::try_from(self.fd())
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.as_ref().unwrap().handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}

impl<HANDLER: Handler> ConnectorNng<HANDLER> {
    pub fn bind(self) -> ConnectorNng<HANDLER> {
        self.socket.listen(&self.endpoint).unwrap();
        self
    }

    pub fn builder() -> ConnectorNngBuilder<HANDLER> {
        ConnectorNngBuilder {
            endpoint: "".to_string(),
            handler: None,
        }
    }
}

impl<H: Handler> Sender for ConnectorNng<H> {
    fn send(&self, data: Vec<u8>) {
        self.socket
            // .send(data.as_bytes())
            .send("Ferris1".as_bytes())
            .expect("client failed sending data");
    }
}

pub struct ConnectorNngBuilder<HANDLER: Handler> {
    endpoint: String,
    handler: Option<Box<HANDLER>>,
}

impl<HANDLER: Handler> ConnectorNngBuilder<HANDLER> {
    pub fn with_handler(mut self, handler: HANDLER) -> Self {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }

    // pub fn with_xtype(mut self, xtype: SocketType) -> Self {
    //     self.xtype = xtype;
    //     self
    // }

    // pub fn with_context(mut self, context: Box<Context>) -> Self {
    //     self.context = context;
    //     self
    // }

    // pub fn connect(self) -> Connector {
    //     self.socket
    //         .connect(&self.endpoint)
    //         .expect(format!("failed connecting to {}", &self.endpoint).as_str());
    //
    //     self
    // }

    pub fn build(self) -> ConnectorNng<HANDLER> {
        ConnectorNng {
            endpoint: self.endpoint,
            handler: self.handler,
            socket: Socket::new(Protocol::Rep0).unwrap(),
        }
    }
}

mod tests {
    use std::net::TcpListener;
    use std::io::{Read, Write};
    use std::thread;
    use zmq::{DEALER, ROUTER};
    use polling::Event;

    use super::*;

    #[test]
    fn test() {
        let context = ContextBuilder::new().build(); //TODO Use From trait instead of new
        let connector_context = context.clone();

        // let dealer_server = ConnectorBuilder::new()
        //     .with_context(context)
        //     .with_xtype(zmq::DEALER)
        //     .with_endpoint("inproc://test".to_string())
        //     .with_handler(|data| {
        //         let result = String::from_utf8(data);
        //         println!("received data {:?}", result);
        //     })
        //     .build()
        //     .bind();

        let server_handle = thread::spawn(move || {
            // let poller = polling::Poller::new().unwrap();
            // poller.add(&socket, Event::readable(key));
            // poller.poll();
        });

        // let _client = ConnectorBuilder::new()
        //     .with_context(connector_context)
        //     .with_xtype(zmq::DEALER)
        //     .with_endpoint("inproc://test".to_string())
        //     .build()
        //     .connect()
        //     .send(b"test".to_vec());

        // assert_eq!(json_result, std::str::from_utf8(&json_buffer).unwrap());

        // f: impl Fn(i32, PcapPacket)

        // server_handle.join().unwrap();
    }

    trait Parser {
        fn parse(&self);
    }

    struct PlainParser;

    impl Parser for PlainParser {
        fn parse(&self) {
            println!("Hello!")
        }
    }

    struct Printer {
        parsers: Vec<Box<dyn Parser>>,
    }

    impl Printer {
        pub fn print(&self) {
            self.parsers.iter()
                .for_each(|parser| parser.parse())
        }
    }

    #[test]
    fn test_handlers() {
        let plain_parser = PlainParser;

        let printer = Printer { parsers: vec![Box::new(plain_parser)] };
        printer.print();
    }
}
