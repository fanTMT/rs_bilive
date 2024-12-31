use crate::{
    api::control::{search_paly, skip_next},
    openlive::proto::{
        CDM, CGuard, CLike, CSendGift, CSuperChat, CSuperChatDel, LIVE_OPEN_PLATFORM_SEND_GIFT,
        LiveOpenPlatformCmd, RawProto,
    },
};

use flate2::write::ZlibDecoder;
use std::io::Write;

/// 消息解析
pub async fn msgthead(bytes: Vec<u8>) {
    if let Ok(proto) = RawProto::try_from(bytes) {
        // println!("返回的消息:{:?}", proto);
        if proto.version == 2 {
            //处理压缩
            let mut writer = Vec::new();
            let mut z = ZlibDecoder::new(writer);
            let r = z.write_all(&proto.body);
            if r.is_err() {
                return;
            }
            let r = z.finish();
            if r.is_err() {
                return;
            }
            writer = r.unwrap();
            //递归消息处理
            // handle(writer, raw_handles, op_handles, cmd_handles, params.clone());
            return;
        }
        // for op in op_handles.read().await.iter() {
        //     let proto: RawProto = proto.clone();
        //     let params = params.clone();
        //     op.handle(proto, params).await;
        // }
        if proto.operation == 5 {
            match String::from_utf8(proto.body) {
                Ok(json) => match serde_json::from_str::<serde_json::Value>(&json) {
                    Ok(v) => {
                        // println!("数据包{:?}", v);
                        if let Some((_, v)) = v.as_object().and_then(|m| m.iter().next()) {
                            if let Some(cmd) = v.as_str() {
                                match cmd {
                                    // 有人发送弹幕
                                    "LIVE_OPEN_PLATFORM_DM" => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CDM>>(&json)
                                        {
                                            let dm = pcmd.data.msg;
                                            let re =
                                                regex::Regex::new(r"点歌\s+(\S+)(?:\s+(\S+))?")
                                                    .unwrap();
                                            let Some(a) = re.captures(&dm) else {
                                                if dm.starts_with("切歌") {
                                                    skip_next();
                                                } else {
                                                    // 不是点歌开头 后期加入读弹幕
                                                    println!("no match!{:?}", dm);
                                                }
                                                return;
                                            };
                                            let musicname =
                                                a.get(1).unwrap().as_str().trim().to_string();
                                            let singer = a.get(2).map_or("", |m| m.as_str());
                                            // println!("c{:#?}", c);
                                            let _ = search_paly(pcmd.data.uname, musicname, singer);
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
