use async_trait::async_trait;

use crate::models::order::Order;

#[async_trait]
pub trait Exchange {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    // 账户相关
    async fn get_balance(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_positions(&self) -> Result<(), Box<dyn std::error::Error>>;

    // 订单相关
    async fn place_order(&self, order: &Order) -> Result<(), Box<dyn std::error::Error>>;
    async fn cancel_order(&self, order_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_order(&self, order_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_orders(&self) -> Result<(), Box<dyn std::error::Error>>;

    // 行情相关
    async fn subscribe_market(&self, symbol: Vec<String>)
    -> Result<(), Box<dyn std::error::Error>>;
}
