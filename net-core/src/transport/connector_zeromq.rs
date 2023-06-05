use std::sync::Arc;

use super::sockets::{self, Pub};

use super::sockets::{
    Receiver,
    Sender,
    Handler,
};

pub struct ConnectorZMQ<HANDLER: Handler> {
    endpoint: String,
    handler: Arc<HANDLER>,
    _context: zmq::Context,
    socket: zmq::Socket
}

impl<HANDLER: Handler> Receiver for ConnectorZMQ<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv_bytes(0)
            .expect("connector failed receiving data")
    }
}

impl<HANDLER: Handler> Pub for ConnectorZMQ<HANDLER> {
    fn set_topic(&self, _topic: &[u8]){
        log::error!("can't set a topic for a non pub connector");
    }
}


impl<HANDLER: Handler> Sender for ConnectorZMQ<HANDLER> {
    fn send(&self, data: &[u8]) {
        self.socket.send(data, 0)
            .expect("client failed sending data");
    }

    fn get_pub(&self) -> Option<&dyn sockets::Pub> {
        None
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorZMQ<HANDLER> {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.socket.get_fd().unwrap()
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}

impl<HANDLER: Handler> ConnectorZMQ<HANDLER> {
    pub fn bind(self) -> Self {
        self.socket.bind(&self.endpoint)
            .expect("couldn't bind a connector");
        self
    }
    pub fn connect(self) -> Self {
        self.socket.connect(&self.endpoint)
            .expect("couldn't establish a connection");
        self
    }
    pub fn into_inner(self) -> Arc<Self> {
        Arc::from(self)
    }
    pub fn builder() -> ConnectorZmqBuilder<HANDLER> {
        ConnectorZmqBuilder::new()
    }
    pub fn get_socket(&self) -> &zmq::Socket {
        &self.socket
    }
}

pub struct ConnectorZmqBuilder<HANDLER: Handler> {
    context: zmq::Context,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
    socket: Option<zmq::Socket> 
}

impl<HANDLER: Handler> ConnectorZmqBuilder<HANDLER> {
    pub fn new() -> Self {
        ConnectorZmqBuilder {
            context: zmq::Context::new(),
            endpoint: None,
            handler: None,
            socket: None
        }
    }
    pub fn with_handler(mut self, handler: HANDLER) -> Self {
        self.handler = Some(Arc::new(handler));
        self
    }
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    fn build(self) -> ConnectorZMQ<HANDLER> {
        ConnectorZMQ { 
            endpoint: self.endpoint.unwrap(),
            handler: self.handler.unwrap(),
            socket: self.socket.unwrap(),
            _context: self.context
        }
    } 
    pub fn build_dealer(mut self) -> ConnectorZMQ<HANDLER> {
        self.socket = Some(self.context.socket(zmq::DEALER).unwrap());
        self.build()
    }
    pub fn build_router(mut self) -> ConnectorZMQ<HANDLER> {
        self.socket = Some(self.context.socket(zmq::ROUTER).unwrap());
        self.build()
    }
}

