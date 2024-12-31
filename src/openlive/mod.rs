use std::sync::Arc;

use anyhow::Ok;
use auth_request::openliveauth;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::oneshot::Sender;

pub mod auth_request;
pub mod proto;
pub mod ws;

pub fn urljoin(str: &str) -> String {
    format!("{}{}", API_HOST, str)
}

const API_HOST: &str = "https://live-open.biliapi.com";
/// 项目开启
const API_START: &str = "/v2/app/start";
/// 项目关闭
const API_END: &str = "/v2/app/end";
/// 项目心跳
const API_HEARBEAT: &str = "/v2/app/heartbeat";
/// 项目批量心跳
const API_BATCH_HEARTBEAT: &str = "/v2/app/batchHeartbeat";

const OP_HEARTBEAT: usize = 2;
const OP_HEARTBEAT_REPLY: usize = 3;
const OP_SEND_SMS_REPLY: usize = 5;
const OP_AUTH: usize = 7;
const OP_AUTH_REPLY: usize = 8;

#[derive(Debug)]
pub struct OpenLive {
    pub is_run: bool,
    /// [主播身份码] 只需更换这个
    pub code: String,
    /// AccessKeyId
    pub access_key_id: String,
    /// access_secret_key
    pub access_secret_key: String,
    /// https://open-live.bilibili.com/miniapp/overview/1733438858944
    pub app_id: usize,
    /// 场次id,心跳key----------------------------------------------
    pub game_id: String,
    /// 连接字符串
    pub auth_body: Option<String>,
    /// wss长连接地址
    pub wsaddr: Option<String>,
}

impl OpenLive {
    pub fn default(code: &str) -> OpenLive {
        OpenLive {
            is_run: false,
            code: code.to_string(),
            access_key_id: "3mwTEo1aKN0EiXDlRcGSbMH6".to_string(),
            access_secret_key: "FZZ248WMjuIEHJDBe7tx9om2krP1mI".to_string(),
            app_id: 1733438858944,
            game_id: String::new(),
            auth_body: None,
            wsaddr: None,
        }
    }
    pub fn new(
        code: &str,
        mut access_key_id: &str,
        mut access_secret_key: &str,
        mut app_id: usize,
    ) -> OpenLive {
        if access_key_id.is_empty() && access_secret_key.is_empty() && app_id == 0 {
            access_key_id = "3mwTEo1aKN0EiXDlRcGSbMH6";
            access_secret_key = "FZZ248WMjuIEHJDBe7tx9om2krP1mI";
            app_id = 1733438858944;
        }
        OpenLive {
            is_run: false,
            code: code.to_string(),
            access_key_id: access_key_id.to_string(),
            access_secret_key: access_secret_key.to_string(),
            app_id,
            game_id: String::new(),
            auth_body: None,
            wsaddr: None,
        }
    }
    // 项目关闭
    pub async fn end(&mut self) {
        let j = json!({"app_id":self.app_id.clone(), "game_id":self.game_id});
        let json = serde_json::to_string(&j).unwrap();
        let res = openliveauth(
            &urljoin(API_END),
            json,
            self.access_key_id.clone(),
            self.access_secret_key.clone(),
        )
        .await
        .expect("关闭访问失败！！！");
        let v = res
            .json::<serde_json::Value>()
            .await
            .expect("关闭项目解析失败！！！");
        println!("{:?}", v);
    }
    // 项目批量心跳
    pub async fn batch_heartbeat_start(&mut self) {}
    // 项目心跳
    pub async fn heartbeat_start(&mut self) -> anyhow::Result<()> {
        // let a = Arc::clone(&self.game_id);
        let access_key_id = Arc::new(Mutex::new(self.access_key_id.clone()));
        let id = Arc::new(Mutex::new(self.access_secret_key.clone()));
        let game_id = Arc::new(Mutex::new(self.game_id.clone()));
        tokio::spawn(async move {
            loop {
                let gameid = game_id.lock().await.to_string();
                let j = json!({"game_id":gameid});
                let json = serde_json::to_string(&j).unwrap();
                // println!("心跳：{}{}", json, API_HEARBEAT);
                let res = openliveauth(
                    &urljoin(API_HEARBEAT),
                    json,
                    access_key_id.lock().await.to_string(),
                    id.lock().await.to_string(),
                )
                .await
                .expect("心跳请求失败！");
                let _a = res
                    .json::<serde_json::Value>()
                    .await
                    .expect("心跳解析失败!");
                // println!("心跳包:{:#?}", _a);
                println!("心跳包:{:#?}", _a.as_object().unwrap()["message"]);
                tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            }
        });
        Ok(())
    }
    /// 启动 项目
    pub async fn start(&mut self) -> Result<&mut OpenLive, anyhow::Error> {
        let j = json!({"code":self.code.clone(), "app_id":self.app_id});
        let json = serde_json::to_string(&j).unwrap();
        let res = openliveauth(
            &urljoin(API_START),
            json,
            self.access_key_id.clone(),
            self.access_secret_key.clone(),
        )
        .await?;
        let a = res.json::<StartOut>().await.expect("请检查【身份码】");
        // println!("{:#?}", a);
        if a.code == 0 {
            // println!("game_id:{:?}", a.data.game_info.game_id);
            self.is_run = true;
            self.game_id = a.data.game_info.game_id;
            self.auth_body = Some(a.data.websocket_info.auth_body);
            let a = a.data.websocket_info.wss_link.first().unwrap();
            self.wsaddr = Some(a.to_string());
            // self.channel = Some(Arc::new(Mutex::new(serder)));
            // println!("主要配置:{:#?}", self); // -----------------------------------------------------------------------------
            // 启动项目心跳包
            let _h = self.heartbeat_start().await;
            // 启动直播间长连
            let _ws = self.new_ws().await;

            Ok(self)
        } else {
            Err(anyhow::anyhow!("【Error】{}", a.message))
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct StartOut {
    code: usize,
    message: String,
    data: StartOutData,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StartOutData {
    //  场次信息
    pub game_info: GameInfo,
    //  长连信息
    pub websocket_info: WebsocketInfo,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WebsocketInfo {
    //  长连使用的请求json体 第三方无需关注内容,建立长连时使用即可
    pub auth_body: String,
    //  wss 长连地址
    pub wss_link: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameInfo {
    game_id: String,
}
