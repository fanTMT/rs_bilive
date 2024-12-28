use std::process::Command;

// 弹幕接入lxmusic
// pub fn dm2lxmusic(name: String, str: String) {
//     println!("{:?} 点歌 {:?}", name, str);
//     let a = search_paly(str);
// }

// #[cfg()]
pub fn search_paly(name: String, str: String) -> Result<String, anyhow::Error> {
    // lx-music lxmusic://music/searchPlay/%s
    let msg = urlencoding::decode(&str).unwrap();
    let mut binding = Command::new("lx-music");
    let cmd = binding.arg(format!("lxmusic://music/searchPlay/{}", msg));
    cmd.output().unwrap();
    println!("{}点歌:【{}】", name, str);
    Ok(format!("点歌:【{}】", str))
}
