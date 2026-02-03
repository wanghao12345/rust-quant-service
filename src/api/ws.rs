use std::net::SocketAddr;

use crate::state::AppState;
use axum::{
    body::Bytes,
    extract::{
        ConnectInfo, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, headers::UserAgent};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};

pub async fn ws_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "Unknown browser".to_string()
    };

    println!("{} connected", user_agent);
    ws.on_upgrade(move |socket| handle_socket(socket, state.ws_service))
}

async fn handle_socket(socket: WebSocket, ws_service: crate::services::ws_service::WsService) {
    // 注册 WebSocket 连接
    let (connection_id, mut rx) = ws_service.register();
    println!("WebSocket 连接已注册，ID: {}", connection_id);
    
    // 订阅广播通道
    let mut bcast_rx = ws_service.subscribe();
    
    // 分割 socket 为 sender 和 receiver
    let (mut sender, mut receiver) = socket.split();
    
    // 发送初始 ping
    if sender
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged client...");
    } else {
        println!("Could not send ping!");
        ws_service.unregister(connection_id);
        return;
    }
    
    // 创建定时发送 ping 的定时器
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    
    // 在同一个任务中同时处理消息接收、定时 ping 和广播消息
    loop {
        tokio::select! {
            // 处理消息接收
            Some(Ok(message)) = receiver.next() => {
                match message {
                    Message::Text(text) => {
                        println!("收到文本消息：{}", text);
                        // 转换为 String
                        let text_str = text.to_string();
                        // 发送相同的消息回去
                        // if let Err(e) = sender.send(Message::Text(text)).await {
                        //     println!("发送消息失败：{}", e);
                        //     break;
                        // }
                        // 广播消息给所有客户端
                        ws_service.broadcast(text_str);
                    }
                    Message::Binary(binary) => {
                        println!("收到二进制消息：{:?}", binary);
                    }
                    Message::Ping(ping) => {
                        println!("收到 Ping 帧：{:?}", ping);
                        // 发送 Pong 响应
                        if let Err(e) = sender.send(Message::Pong(ping)).await {
                            println!("发送 Pong 失败：{}", e);
                            break;
                        }
                    }
                    Message::Pong(pong) => {
                        println!("收到 Pong 帧：{:?}", pong);
                    }
                    Message::Close(close_frame) => {
                        match &close_frame {
                            Some(close_frame) => {
                                println!(
                                    "连接关闭：{} ({:?})",
                                    close_frame.code,
                                    close_frame.reason.as_str()
                                );
                            }
                            None => {
                                println!("关闭帧无原因");
                            }
                        }
                        // 发送关闭消息
                        if let Err(e) = sender.send(Message::Close(close_frame)).await {
                            println!("发送关闭消息失败：{}", e);
                        }
                        break;
                    }
                }
            },
            // 处理定时 ping
            _ = interval.tick() => {
                if let Err(e) = sender.send(Message::Ping(Bytes::from_static(b"ping"))).await {
                    println!("发送心跳失败: {}", e);
                    break;
                }
                println!("发送心跳消息");
            },
            // 处理广播消息
            Ok(msg) = bcast_rx.recv() => {
                println!("收到广播消息：{}", msg);
                if let Err(e) = sender.send(Message::Text(msg.into())).await {
                    println!("发送广播消息失败：{}", e);
                    break;
                }
            },
            // 处理从 mpsc 通道接收的消息
            Some(msg) = rx.recv() => {
                println!("收到通道消息");
                if let Err(e) = sender.send(msg).await {
                    println!("发送通道消息失败：{}", e);
                    break;
                }
            }
        }
    }
    
    // 注销 WebSocket 连接
    ws_service.unregister(connection_id);
    println!("WebSocket 连接已关闭并注销，ID: {}", connection_id);
}
