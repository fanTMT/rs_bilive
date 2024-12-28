use super::{OpenLive, proto::RawProto};
use crate::danmu::msgthead;

use futures::{SinkExt, StreamExt};
use reqwest_websocket::RequestBuilderExt;

impl OpenLive {
    pub async fn new_ws(&mut self) -> Result<(), anyhow::Error> {
        let server_url = self.wsaddr.clone().unwrap();

        // Creates a GET request, upgrades and sends it.
        let response = match reqwest::Client::default()
            .get(server_url)
            .upgrade() // Prepares the WebSocket upgrade.
            .send()
            .await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("WebSocket connection failed {} {:?}", e, self.wsaddr);
                return Ok(());
            }
        };
        let websocket = response.into_websocket().await?;

        let (mut writer, read) = websocket.split();
        // 发送AUTH包
        writer
            .send(reqwest_websocket::Message::Binary(
                RawProto::new(7, self.auth_body.clone().unwrap().as_bytes().to_vec()).into(),
            ))
            .await?;
        // 接收消息
        tokio::spawn(async move {
            let mut reader = read;
            while let Some(message) = reader.next().await {
                match message {
                    Ok(msg) => {
                        if let reqwest_websocket::Message::Binary(bytes) = msg {
                            // println!("接收的：{:?}", String::from_utf8_lossy(&bytes));
                            // 消息解析
                            msgthead(bytes).await;
                        } else if let reqwest_websocket::Message::Ping(_p) = msg {
                        } else {
                            eprintln!("No Binary Data {:?}", msg);
                        }
                    }
                    Err(e) => eprintln!("Failed to receive {e}"),
                }
            }
        });
        // 发送心跳
        tokio::spawn(async move {
            let mut writer = writer;
            loop {
                println!("发送心跳包");
                let proto = RawProto::new(2, Vec::new());
                let result = writer
                    .send(reqwest_websocket::Message::Binary(proto.clone().into()))
                    .await;
                if result.is_err() {
                    eprintln!("Failed to send message {:?}", proto);
                }
                tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            }
        });

        Ok(())
    }
}
