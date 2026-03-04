mod connection;

use std::io;
use tokio::net::TcpListener;
use tracing::info;

use agni::config::Config;
use agni::store::Store;

pub struct Server {
    listener: TcpListener,
    store: Store,
}

impl Server {
    pub async fn new(config: &Config) -> io::Result<Self> {
        let listener = TcpListener::bind(config.addr()).await?;
        info!("listening on {}", config.addr());
        Ok(Self {
            listener,
            store: Store::new(),
        })
    }

    pub async fn run(&self) -> io::Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            info!("new connection: {}", addr);
            let store = self.store.clone();
            tokio::spawn(async move {
                connection::handle(socket, store).await;
            });
        }
    }
}
