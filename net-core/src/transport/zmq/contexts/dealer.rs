use crate::transport::sockets::Context;

#[derive(Default, Clone)]
pub struct DealerContext {
    context: zmq::Context,
}

impl Context for DealerContext {
    type S = zmq::Socket;
    type C = zmq::Context;

    fn create_socket(&self) -> Self::S {
        self.context.socket(zmq::DEALER).unwrap()
    }
    fn get_context(&self) -> Self::C { self.context.clone() }
}