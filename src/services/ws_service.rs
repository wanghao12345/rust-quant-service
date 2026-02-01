use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::extract::ws::Message;
use tokio::sync::{broadcast, mpsc};
use tracing::info;


/// WebSocket 服务
#[derive(Debug, Clone)]
pub struct WsService {
    /// 存储所有 WebSocket 连接的 Sender
    senders: Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>,
    /// 广播通道, 用于向所有连接的客户端发送消息
    bcast_tx: broadcast::Sender<String>,
}

impl WsService {
    /// 创建新的 WebSocket 服务
    pub fn new() -> Self {
        let bcast_tx = broadcast::channel(100).0;
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
            bcast_tx,
        }
    }

    /// 注册新的 WebSocket 连接
    pub fn register(&self) -> (usize, mpsc::UnboundedReceiver<Message>) {
        let id = self.generate_id();
        let (tx, rx) = mpsc::unbounded_channel();
        self.senders.lock().unwrap().insert(id, tx);
        (id, rx)
    }

    /// 广播消息到所有连接的客户端
    pub fn broadcast(&self, msg: String) {
        self.bcast_tx.send(msg).unwrap();
    }

    /// 注销指定 ID 的 WebSocket 连接
    pub fn unregister(&self, id: usize) {
        self.senders.lock().unwrap().remove(&id);
        info!("已注销 WebSocket 连接 ID: {}", id);
    }

    /// 订阅广播通道
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.bcast_tx.subscribe()
    }


    /// 生成新的连接 ID
    fn generate_id(&self) -> usize {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize
    }



}