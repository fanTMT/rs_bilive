use std::process::Command;

// #[cfg()]
pub fn search_paly(str: String) -> Result<String, anyhow::Error> {
    // lx-music lxmusic://music/searchPlay/%s
    let msg = urlencoding::decode(&str).unwrap();
    let mut binding = Command::new("lx-music");
    let cmd = binding.arg(format!("lxmusic://music/searchPlay/{}", msg));
    cmd.output().unwrap();
    println!("点歌:【{}】", str);
    Ok(format!("点歌:【{}】", str))
}
