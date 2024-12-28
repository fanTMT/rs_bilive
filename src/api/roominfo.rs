/// 获取真实房间号
// https://api.live.bilibili.com/room/v1/Room/room_init?id=6
use serde::{Deserialize, Serialize};

/// 获取真实房间号
pub async fn getroomid(id: String) -> Result<RoomInit, anyhow::Error> {
    let client = reqwest::Client::new();
    let api = format!(
        "https://api.live.bilibili.com/room/v1/Room/room_init?id={}",
        id
    );
    let res = client.get(api)
        .timeout(std::time::Duration::from_secs(30))
        .header("User-Agent","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0")
        .send()
        .await?
        .json::<RoomInit>()
        .await?;
    if res.code == 0 {
        Ok(res)
    } else {
        eprintln!("{:?}", res.msg);
        Err(anyhow::anyhow!("error "))
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RoomInit {
    pub code: usize,
    pub msg: String,
    pub data: RoomInitData,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RoomInitData {
    pub room_id: usize,
}
