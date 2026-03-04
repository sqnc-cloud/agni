use bytes::Bytes;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tracing::error;

#[derive(Parser)]
#[command(name = "agni-client", about = "CLI client for agni server")]
struct Cli {
    /// Server host
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Server port
    #[arg(long, default_value_t = 6379)]
    port: u16,

    /// Command to send (e.g. PING)
    command: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);

    let socket = TcpStream::connect(&addr).await.unwrap_or_else(|e| {
        error!("could not connect to {}: {}", addr, e);
        std::process::exit(1);
    });

    let (reader, writer) = socket.into_split();
    let mut framed_read = FramedRead::new(reader, LengthDelimitedCodec::new());
    let mut framed_write = FramedWrite::new(writer, LengthDelimitedCodec::new());

    framed_write
        .send(Bytes::from(cli.command.into_bytes()))
        .await
        .unwrap_or_else(|e| {
            error!("failed to send command: {}", e);
            std::process::exit(1);
        });

    match framed_read.next().await {
        Some(Ok(frame)) => println!("{}", String::from_utf8_lossy(&frame)),
        Some(Err(e)) => error!("{}", e),
        None => error!("connection closed with no response"),
    }
}
