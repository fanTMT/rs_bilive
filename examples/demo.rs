use std::env;

use bilive_danmu_core::openlive;
use clap::{Command, arg};
use dotenv::dotenv;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Command::new("点歌")
        .version("0.1.0")
        .author("番茄是水果")
        .about("this is the short about")
        .long_about("就是一个普通的点歌软件!!!")
        .arg(
            arg!(-c --code <CODE>)
                .value_parser(clap::value_parser!(String))
                .help("获取身份码 https://play-live.bilibili.com"),
        )
        .get_matches();
    // 获取身份码 优先识别 env 后识别 -c | --code 命令行参数
    let code = match env::var("CODE") {
        Ok(env) => env,
        Err(_) => cli
            .get_one::<String>("code")
            .unwrap_or(&get_input())
            .to_string(),
    };
    // 官方开源方式
    // 鉴权
    // 开启项目
    // 心跳
    let mut op: openlive::OpenLive = openlive::OpenLive::new(
        &code,
        "3mwTEo1aKN0EiXDlRcGSbMH6",
        "FZZ248WMjuIEHJDBe7tx9om2krP1mI",
        1733438858944,
    );
    // 直播间长连
    // 等待消息
    op.start().await.expect("Error:启动失败!!!");
    // 关闭项目
    // 当按下 Ctrl+C 时，这个 future 会完成
    let t = tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for ctrl+c");
        println!("Ctrl+C 强制退出!");
        let _a = op.end().await;
    });
    t.await.unwrap();
}

fn get_input() -> String {
    println!("请输入【身份码】：");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("无法读取输入");
    return input;
}
