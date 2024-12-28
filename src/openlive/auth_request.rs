use crate::api::{current_timestamp_10, md5, sha256};

// pub trait Openliveauth {
//     fn openliveauth(
//         &self,
//         url: &str,
//         json: String,
//     ) -> impl std::future::Future<Output = Result<reqwest::Response, anyhow::Error>> + Send;
//     fn gen_authorization(&self, md5: String, uuid: String) -> String;
// }
// impl Openliveauth for OpenLive {

// impl OpenLive {
//     /// 公共请求 连接 数据 key_id secret_key
//     async fn openliveauth(
//         &self,
//         url: &str,
//         json: String,
//         access_key_id: String,
//         access_secret_key: String,
//     ) -> Result<reqwest::Response, anyhow::Error> {
//         let md5data = md5(json.clone());
//         let time = current_timestamp_10();
//         let uuid = uuid::Uuid::new_v4().to_string();
//         let client = reqwest::Client::new();
//         let res: reqwest::Response = client.post(url)
//     .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")
//     .header("Accept", "application/json")
//     .header("Content-Type", "application/json")
//     .header("x-bili-content-md5", md5data.clone())
//     .header("x-bili-timestamp", time)
//     .header("x-bili-signature-version", "1.0")
//     .header("x-bili-signature-nonce", uuid.clone())
//     .header("x-bili-signature-method", "HMAC-SHA256")
//     .header("x-bili-accesskeyid", access_key_id.clone())
//     .header("Authorization", self.gen_authorization(md5data,uuid.clone(),access_secret_key))
//     .body(json)
//     .send()
//     .await?;
//         Ok(res)
//     }

//     fn gen_authorization(&self, md5: String, uuid: String, access_secret_key: String) -> String {
//         let mut pre_authorization = String::new();
//         // FZZ248WMjuIEHJDBe7tx9om2krP1mI
//         pre_authorization.push_str(&format!("x-bili-accesskeyid:{}\n", self.access_key_id));
//         pre_authorization.push_str(&format!("x-bili-content-md5:{}\n", md5));
//         pre_authorization.push_str("x-bili-signature-method:HMAC-SHA256\n");
//         pre_authorization.push_str(&format!("x-bili-signature-nonce:{}\n", uuid));
//         pre_authorization.push_str("x-bili-signature-version:1.0\n");
//         pre_authorization.push_str(&format!("x-bili-timestamp:{}", current_timestamp_10()));
//         // println!("pre_authorization:{:?}", pre_authorization);
//         sha256(pre_authorization, access_secret_key.clone())
//     }
// }

pub async fn openliveauth(
    url: &str,
    json: String,
    access_key_id: String,
    access_secret_key: String,
) -> Result<reqwest::Response, anyhow::Error> {
    let md5data = md5(json.clone());
    // println!("{:?}--{:?}", json, md5data);
    let time = current_timestamp_10();
    let uuid = uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    let res: reqwest::Response = client.post(url)
            .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("x-bili-content-md5", md5data.clone())
            .header("x-bili-timestamp", time)
            .header("x-bili-signature-version", "1.0")
            .header("x-bili-signature-nonce", uuid.clone())
            .header("x-bili-signature-method", "HMAC-SHA256")
            .header("x-bili-accesskeyid", access_key_id.clone())
            .header("Authorization", gen_authorization(md5data,uuid.clone(),access_key_id,access_secret_key))
            .body(json)
            .send()
.await?;
    Ok(res)
}

fn gen_authorization(
    md5: String,
    uuid: String,
    access_key_id: String,
    access_secret_key: String,
) -> String {
    let mut pre_authorization = String::new();
    // FZZ248WMjuIEHJDBe7tx9om2krP1mI
    pre_authorization.push_str(&format!("x-bili-accesskeyid:{}\n", access_key_id));
    pre_authorization.push_str(&format!("x-bili-content-md5:{}\n", md5));
    pre_authorization.push_str("x-bili-signature-method:HMAC-SHA256\n");
    pre_authorization.push_str(&format!("x-bili-signature-nonce:{}\n", uuid));
    pre_authorization.push_str("x-bili-signature-version:1.0\n");
    pre_authorization.push_str(&format!("x-bili-timestamp:{}", current_timestamp_10()));
    // println!("pre_authorization:{:?}", pre_authorization);
    sha256(pre_authorization, access_secret_key.clone())
}
