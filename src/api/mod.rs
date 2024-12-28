pub mod control;
pub mod roominfo;

use cryptocol::hash;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::SystemTime;

/// md5 加密
pub fn md5(str: String) -> String {
    let mut md5 = hash::MD5::new();
    md5.digest_string(&str);
    md5.get_hash_value_in_string().to_ascii_lowercase()
}

// 获取当前时间戳 10位
pub fn current_timestamp_10() -> u64 {
    // 获取当前系统时间
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();
    // 转换为10位时间戳
    now % 10_000_000_000u64
}

/// sha256 加密
pub fn sha256(str: String, key: String) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut hmac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any length");
    hmac.update(str.as_bytes());
    let a = hmac.finalize().into_bytes();
    format!("{:x}", a)
}
