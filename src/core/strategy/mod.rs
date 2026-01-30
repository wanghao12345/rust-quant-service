use std::{collections::HashMap, task::Context};


/// 策略特征定义
pub trait Strategy {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;

    // 生命周期方法
    fn on_init(&mut self, ctx: &mut Context);
    fn on_start(&mut self, ctx: &mut Context);
    fn on_stop(&mut self, ctx: &mut Context);
    // 事件处理方法
    fn on_tick(&mut self, ctx: &mut Context);
    fn on_bar(&mut self, ctx: &mut Context);
    fn on_order(&mut self, ctx: &mut Context);
    fn on_trade(&mut self, ctx: &mut Context);
}

/// 策略管理器
pub struct StrategyManager {
    strategies: HashMap<String, Box<dyn Strategy>>,
}

impl StrategyManager {
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
        }
    }
    /// 加载策略
    pub async fn load_strategy(&mut self, strategy: Box<dyn Strategy>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    /// 启动策略
    pub async fn start_strategy(&mut self, strategy_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    /// 停止策略
    pub async fn stop_strategy(&mut self, strategy_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}