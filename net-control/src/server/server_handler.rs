use super::aggregator::Aggregator;

#[derive(Clone)]
pub struct ServerHandler {
    aggregator: std::sync::Arc<std::sync::Mutex<Aggregator>>
}

impl ServerHandler {
    pub (super) fn new() -> Self {
        ServerHandler {
            aggregator: std::sync::Arc::new(std::sync::Mutex::new(Aggregator::new()))
        }
    }
}

impl Default for ServerHandler {
    fn default() -> Self {
        ServerHandler {
            aggregator: std::sync::Arc::new(std::sync::Mutex::new(Aggregator::default()))
        }
    }
}


#[async_trait::async_trait]
impl russh::server::Handler for ServerHandler {
    type Error = anyhow::Error;

    async fn disconnected(self, session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        Ok((self, session))
    }

    async fn auth_none(self, user: &str) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_password(self, user: &str, password: &str) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_publickey(self, user: &str, public_key: &russh_keys::key::PublicKey) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_succeeded(self, session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        Ok((self, session))
    }

    async fn channel_open_session(self, mut channel: russh::Channel<russh::server::Msg> , session: russh::server::Session) -> Result<(Self, bool, russh::server::Session), Self::Error> {
        match channel.data("\nHello from the CLI!\r\n".as_bytes()).await {
            Ok(_) => (),
            Err(_) => todo!(),
        }

        Ok((self, true, session))
    }

    async fn channel_close(self, channel: russh::ChannelId, mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        session.data(channel, russh::CryptoVec::from("Goodbye, user!".to_string()));
        Ok((self, session))
    }

    async fn data(self, channel: russh::ChannelId, data: &[u8], mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        let mut data_cooked = std::str::from_utf8(data).unwrap().to_string();

        if data_cooked == "\r" {
            data_cooked = ">promt \r\n".to_string();
        }
        
        //For now just echo everything received
        session.data(channel, russh::CryptoVec::from(data_cooked));

        Ok((self, session))
    }
}