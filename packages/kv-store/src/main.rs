use std::{error::Error, sync::Arc};

use kv_store::{
    kv_store::KvStore,
    protocol::{decode, Command, CommandResponse},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

async fn handle_command(buffer: [u8; 4096], size: usize, kv: &Arc<KvStore>) -> CommandResponse {
    match decode(buffer, size) {
        Command::Get(key) => match kv.get(key).await {
            Some(value) => CommandResponse::Success(value.into()),
            None => CommandResponse::NotFound,
        },
        Command::Set(key, value) => {
            kv.set(key, value).await;
            CommandResponse::Success("".into())
        }
        Command::Delete(key) => match kv.delete(key).await {
            Some(value) => CommandResponse::Success(value.into()),
            None => CommandResponse::NotFound,
        },
        Command::Exists(key) => match kv.exists(key).await {
            true => CommandResponse::Success("1".into()),
            false => CommandResponse::Success("0".into()),
        },
        Command::Keys => CommandResponse::Success(kv.keys().await.join(" ").into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let kv = Arc::new(KvStore::new());
    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("Failed to bind to port 8080");

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let kv_clone = kv.clone();
        tokio::spawn(async move {
            loop {
                let mut buffer = [0; 4096];
                let size = socket
                    .read(&mut buffer)
                    .await
                    .expect("Failed to read from socket");

                if size == 0 {
                    continue;
                }

                let result = handle_command(buffer, size, &kv_clone).await;

                socket
                    .write(result.into_bytes().as_slice())
                    .await
                    .expect("Failed to write response");
            }
        });
    }
}
