use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tracing::error;

use agni::protocol::{Command, Response};
use agni::store::Store;

pub async fn handle(socket: TcpStream, _store: Store) {
    let (reader, writer) = socket.into_split();
    let mut framed_read = FramedRead::new(reader, LengthDelimitedCodec::new());
    let mut framed_write = FramedWrite::new(writer, LengthDelimitedCodec::new());

    while let Some(result) = framed_read.next().await {
        match result {
            Ok(frame) => {
                let response = match Command::from_bytes(&frame) {
                    Command::Ping => Response::Pong,
                    Command::Unknown(cmd) => Response::Error(format!("unknown command '{}'", cmd)),
                };

                if framed_write
                    .send(Bytes::from(response.to_bytes()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Err(e) => {
                error!("connection error: {}", e);
                break;
            }
        }
    }
}
