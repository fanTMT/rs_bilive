use bilive_danmu_core::openlive;
use tokio::signal;

#[tokio::main]
async fn main() {
    // 官方开源方式
    // 鉴权
    // 开启项目
    // 心跳
    let mut op: openlive::OpenLive = openlive::OpenLive::new(
        "E836LNYJRY0W6".to_string(),
        "3mwTEo1aKN0EiXDlRcGSbMH6".to_string(),
        "FZZ248WMjuIEHJDBe7tx9om2krP1mI".to_string(),
        1733438858944,
    );
    op.start().await.expect("Error:启动失败!!!");
    // 直播间长连
    // 关闭项目
    // 等待消息
    // 当按下 Ctrl+C 时，这个 future 会完成
    let t = tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for ctrl+c");
        println!("Ctrl+C 强制退出!");
        let _a = op.end().await;
    });
    t.await.unwrap();
}
