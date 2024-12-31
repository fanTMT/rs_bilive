use std::process::Command;

use serde_json::json;

// 弹幕接入lxmusic https://lxmusic.toside.cn/desktop/scheme-url

// music/searchPlay
pub fn search_paly(name: String, str: String, singer: &str) -> Result<String, anyhow::Error> {
    // 搜索音乐并播放
    // :param name:（歌曲名，必须）
    // :param singer:（歌手，可选）
    // :param albumName:（专辑名，可选）
    // :param interval:（时长，xx:xx 的形式，可选）
    // :param playLater:（是否稍后播放，可选，默认 false 立即播放）
    // music/searchPlay
    // lx-music lxmusic://music/searchPlay/%s
    let json = json!({"name":str,"singer":singer,"albumName":"","interval":"","playLater":false});
    let data = serde_json::to_string(&json).unwrap();
    let msg = urlencoding::encode(&data);
    // println!("{}<<<>>>{}", json, msg);
    let mut binding = Command::new("lx-music");
    let cmd = binding.arg(format!("lxmusic://music/searchPlay?data={}", msg));
    cmd.output().unwrap();
    // println!("lx-music lxmusic://music/searchPlay?data={}", msg);
    println!("{}点歌:【{}】", name, str);
    Ok(format!("点歌:【{}】", str))
}

// player/skipNext 下一曲

pub fn skip_next() {
    let mut binding = Command::new("lx-music");
    let cmd = binding.arg("lxmusic://player/skipNext");
    cmd.output().unwrap();
}
