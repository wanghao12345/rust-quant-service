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
    // State(_state): State<AppState>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "Unknown browser".to_string()
    };

    println!("{} connected", user_agent);
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    // if socket
    //     .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
    //     .await
    //     .is_ok()
    // {
    //     println!("Pinged client...");
    // } else {
    //     println!("Could not send ping!");
    //     return;
    // }

    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(read(receiver, sender));
}

async fn read(mut receiver: SplitStream<WebSocket>, mut sender: SplitSink<WebSocket, Message>) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                println!("收到文本消息：{}", text);
                // 发送相同的消息回去
                if let Err(e) = sender.send(Message::Text(text)).await {
                    println!("发送消息失败：{}", e);
                    break;
                }
            }
            Message::Binary(binary) => {
                println!("收到二进制消息：{:?}", binary);
            }
            Message::Ping(ping) => {
                println!("收到 Ping 帧：{:?}", ping);
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
                break;
            }
        }
    }
    println!("读取循环结束，WebSocket 连接已关闭");
}
