use crate::openlive::proto::{
    CDM, CGuard, CLike, CSendGift, CSuperChat, CSuperChatDel, LIVE_OPEN_PLATFORM_SEND_GIFT,
    LiveOpenPlatformCmd, RawProto,
};

/// 消息解析
pub async fn msgthead(bytes: Vec<u8>) {
    if let Ok(proto) = RawProto::try_from(bytes) {
        // println!("返回的消息:{:?}", proto);
        if proto.version == 2 {}
        if proto.operation == 5 {
            match String::from_utf8(proto.body) {
                Ok(json) => match serde_json::from_str::<serde_json::Value>(&json) {
                    Ok(v) => {
                        println!("{:?}", v);
                        if let Some((_, v)) = v.as_object().and_then(|m| m.iter().next()) {
                            if let Some(cmd) = v.as_str() {
                                match cmd {
                                    // 有人发送弹幕
                                    "LIVE_OPEN_PLATFORM_DM" => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CDM>>(&json)
                                        {
                                            println!("消息{:?}", pcmd.data.msg);
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle.handle_dm(pcmd.data.clone(), params).await;
                                            // }
                                        }
                                    }
                                    // 获取礼物信息
                                    LIVE_OPEN_PLATFORM_SEND_GIFT => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CSendGift>>(
                                                &json,
                                            )
                                        {
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle
                                            //         .handle_send_gift(pcmd.data.clone(), params)
                                            //         .await;
                                            // }
                                        }
                                    }
                                    //获取付费留言
                                    "LIVE_OPEN_PLATFORM_SUPER_CHAT" => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CSuperChat>>(
                                                &json,
                                            )
                                        {
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle
                                            //         .handle_super_chat(pcmd.data.clone(), params)
                                            //         .await;
                                            // }
                                        }
                                    }
                                    //付费留言下线
                                    "LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL" => {
                                        if let Ok(pcmd) = serde_json::from_str::<
                                            LiveOpenPlatformCmd<CSuperChatDel>,
                                        >(
                                            &json
                                        ) {
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle
                                            //         .handle_super_chat_del(
                                            //             pcmd.data.clone(),
                                            //             params,
                                            //         )
                                            //         .await;
                                            // }
                                        }
                                    }
                                    // 付费大航海
                                    "LIVE_OPEN_PLATFORM_GUARD" => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CGuard>>(
                                                &json,
                                            )
                                        {
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle
                                            //         .handle_guard(pcmd.data.clone(), params)
                                            //         .await;
                                            // }
                                        }
                                    }
                                    // 点赞信息 只有房间处于开播中，才会触发点赞事件。
                                    "LIVE_OPEN_PLATFORM_LIKE" => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CLike>>(
                                                &json,
                                            )
                                        {
                                            // for handle in cmd_handles.read().await.iter() {
                                            //     let params = params.clone();
                                            //     handle.handle_like(pcmd.data.clone(), params).await;
                                            // }
                                        }
                                    }
                                    _ => eprintln!("Unkonw Cmd {cmd}"),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Json Decode Error {e} {json}")
                    }
                },
                Err(e) => eprintln!("Body From utf8 Error {e}"),
            }
        }
    }
    // println!("接收的：{:?}", String::from_utf8_lossy(&bytes));
}
