use std::os::unix::io::RawFd;
use std::sync::Arc;

use nng::{Protocol, Socket};
use nng::options::{Options, RecvFd};

use crate::transport::sockets;
use crate::transport::sockets::{Handler, Receiver, Sender};

//TODO Connector Builder should be redesigned as Fluent API with constraints.

pub struct ConnectorNNG<HANDLER> {
    endpoint: String,
    handler: Option<Box<HANDLER>>,
    socket: Socket,
}

impl<HANDLER> Receiver for ConnectorNNG<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv()
            .unwrap()
            .as_slice()
            .to_vec() //note: every time data is coped from stack to the heap!
    }
}

impl<H: Handler> Sender for ConnectorNNG<H> {
    fn send(&self, data: Vec<u8>) {
        self.socket
            .send(&data)
            .expect("client failed sending data");
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorNNG<HANDLER>
{
    fn as_raw_fd(&self) -> RawFd {
        //FIXME RecvFd will be worked only for Protocols that can receive data
        self.socket.get_opt::<RecvFd>().unwrap()
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

impl<HANDLER: Handler> ConnectorNNG<HANDLER> {
    pub fn bind(self) -> ConnectorNNG<HANDLER> {
        self.socket.listen(&self.endpoint).unwrap();
        self
    }

    pub fn connect(self) -> ConnectorNNG<HANDLER> {
        self.socket
            .dial_async(&self.endpoint)
            .expect(format!("failed connecting to {}", &self.endpoint).as_str());

        self
    }

    pub fn into_inner(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub fn builder() -> ConnectorNngBuilder<HANDLER> {
        ConnectorNngBuilder::new()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Proto
{
    Bus,

    // Pair,

    // Pair,

    Pub,

    Pull,

    Push,

    Rep,

    Req,

    Respondent,

    Sub,

    Surveyor,
}

impl Proto {
    pub fn into(proto: Proto) -> Protocol {
        match proto {
            Proto::Bus => { Protocol::Bus0 }
            Proto::Pub => { Protocol::Pub0 }
            Proto::Pull => { Protocol::Pull0 }
            Proto::Push => { Protocol::Push0 }
            Proto::Rep => { Protocol::Rep0 }
            Proto::Req => { Protocol::Req0 }
            Proto::Respondent => { Protocol::Respondent0 }
            Proto::Sub => { Protocol::Sub0 }
            Proto::Surveyor => { Protocol::Surveyor0 }
        }
    }
}

pub struct ConnectorNngBuilder<HANDLER: Handler> {
    endpoint: Option<String>,
    proto: Option<Proto>,
    handler: Option<Box<HANDLER>>,
    topics:  Option<Vec<u8>>
}

impl<HANDLER: Handler> ConnectorNngBuilder<HANDLER> {
    pub fn new() -> ConnectorNngBuilder<HANDLER> {
        ConnectorNngBuilder {
            endpoint: None,
            proto: None,
            handler: None,
            topics: None
        }
    }

    pub fn with_handler(mut self, handler: HANDLER) -> Self {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn with_proto(mut self, proto: Proto) -> Self {
        self.proto = Some(proto);
        self
    }

    pub fn with_topic(mut self, topics: Vec<u8>) -> Self {
        self.topics = Some(topics);
        self 
    }

    pub fn build(self) -> ConnectorNNG<HANDLER> {
        let proto = Proto::into(self.proto.unwrap());
        ConnectorNNG {
            endpoint: self.endpoint.unwrap(),
            handler: self.handler,
            socket: match self.topics {
               Some(topic) => {
                    let socket = Socket::new(proto).unwrap();
                    socket.set_opt::<nng::options::protocol::pubsub::Subscribe>(topic).unwrap();
                    socket
               },
               None => Socket::new(proto).unwrap()
            },
        }
    }
}