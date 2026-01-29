## 项目架构

crypto-quant-system/
├── Cargo.toml
├── Cargo.lock
├── .env
├── docker-compose.yml
├── README.md
├── scripts/
│   ├── setup.sh
│   ├── deploy.sh
│   └── migrate.sh
├── src/
│   ├── main.rs              # 主入口
│   ├── config/              # 配置管理
│   │   ├── mod.rs
│   │   ├── settings.rs
│   │   └── database.rs
│   ├── api/                 # API 路由层
│   │   ├── mod.rs
│   │   ├── v1/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── strategy.rs
│   │   │   ├── trade.rs
│   │   │   ├── risk.rs
│   │   │   └── audit.rs
│   │   └── middleware/      # 中间件
│   │       ├── mod.rs
│   │       ├── auth.rs
│   │       ├── logging.rs
│   │       └── rate_limit.rs
│   ├── core/               # 核心业务逻辑
│   │   ├── mod.rs
│   │   ├── user/
│   │   ├── strategy/
│   │   ├── trading/
│   │   ├── risk/
│   │   ├── market/
│   │   └── audit/
│   ├── exchange/           # 交易所网关
│   │   ├── mod.rs
│   │   ├── traits.rs       # 交易所特性定义
│   │   ├── binance/
│   │   │   ├── mod.rs
│   │   │   ├── rest.rs
│   │   │   └── ws.rs
│   │   └── factory.rs      # 交易所工厂
│   ├── models/            # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── strategy.rs
│   │   ├── order.rs
│   │   ├── position.rs
│   │   └── audit.rs
│   ├── repository/        # 数据访问层
│   │   ├── mod.rs
│   │   ├── user_repo.rs
│   │   ├── strategy_repo.rs
│   │   └── audit_repo.rs
│   ├── services/          # 服务层
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── strategy_service.rs
│   │   ├── trading_service.rs
│   │   ├── risk_service.rs
│   │   └── ws_service.rs  # WebSocket 服务
│   ├── utils/             # 工具函数
│   │   ├── mod.rs
│   │   ├── crypto.rs      # 加密工具
│   │   ├── time.rs
│   │   └── validation.rs
│   └── workers/           # 后台工作线程
│       ├── mod.rs
│       ├── market_worker.rs
│       ├── risk_worker.rs
│       └── cleanup_worker.rs
├── migrations/            # 数据库迁移
│   └── *.sql
├── tests/                # 测试
├── frontend/             # Vue 前端项目
│   ├── package.json
│   ├── src/
│   │   ├── main.ts
│   │   ├── router/
│   │   ├── store/
│   │   ├── views/
│   │   └── components/
│   └── ...
└── docs/                 # 文档




1. 交易所网关（Exchange Gateway）
职责：统一封装各交易所的 REST API 与 WebSocket 流，向上提供一致的接口。

实现：

前期使用 binance-futures-rs 库对接币安合约。

定义 ExchangeTrait，包含 fetch_balance、place_order、subscribe_market 等方法。

通过 配置化 支持未来接入更多交易所（如 OKX、Bybit）。

2. 行情服务（Market Service）
实时行情：通过 WebSocket 接收币安的 K 线、深度、最新成交等数据，并广播给前端与策略引擎。

历史行情：将行情数据持久化到 ClickHouse，供回测与分析使用。

3. 交易引擎（Trading Engine）
订单管理：接收策略引擎的订单请求，经由风控检查后发送至交易所网关。

订单状态跟踪：通过轮询或 WebSocket 更新订单状态，并推送给前端。

资金/持仓同步：定时从交易所同步账户余额与持仓，维护本地一致视图。

4. 风控引擎（Risk Engine）
规则模板：参考 VeighNa 的 RuleTemplate 设计，每个风控规则继承基类，实现 check_allowed 方法。

内置规则：

单笔委托数量/价值上限（防“乌龙指”）。

活动委托数量上限。

每日流控上限（委托、撤单、成交次数）。

重复报单检查。

委托指令合法性检查（价格 tick、数量上下限等）。

执行时机：在订单进入交易引擎前进行同步检查，也可设置定时异步监控。

5. 策略引擎（Strategy Engine）
策略生命周期管理：加载、启动、暂停、停止策略。

回测框架：基于历史行情数据执行回测，计算收益、夏普比率等指标。

实盘执行：策略通过事件驱动（行情、订单回报）产生交易信号，并调用交易引擎下单。

6. 账户管理（Account Management）
数据结构：使用 qifi-rs 提供的标准化数据结构（Account、Position、Order、Trade 等）。

资产视图：聚合多交易所的资产，显示总余额、可用余额、浮动盈亏等。

7. 审计日志（Audit Logging）
操作日志：记录用户登录、策略修改、风控规则调整等关键操作。

交易日志：记录所有订单、成交、资金变动，确保可追溯。

存储：写入 PostgreSQL，并保留至 ClickHouse 供长期分析。

8. 用户与权限（User & Permission）
RBAC 模型：角色（管理员、交易员、观察员）对应不同的功能权限（如策略编辑、风控设置、仅查看）。

API 密钥管理：用户可在前端添加交易所 API 密钥（加密存储），并设置权限范围。

9. 数据存储与缓存
PostgreSQL：存储用户信息、策略配置、风控参数、操作日志。

Redis：缓存行情快照、会话信息、风控计数器。

ClickHouse：存储海量行情、成交、指标数据，支持快速聚合查询。


🏗️ 系统架构概述
采用前后端分离的微服务架构，确保高内聚、低耦合，便于横向扩展与独立部署。

层级	组件	说明
接入层	Nginx / Traefik	负载均衡、SSL 终止、静态资源服务
前端	Vue 3 + TypeScript + Pinia + Element Plus / Ant Design Vue	单页应用，组件化开发
后端	Rust（Axum / Actix-web） + Tokio（异步运行时）	提供 RESTful API 与 WebSocket 推送
数据层	PostgreSQL（交易/账户/审计日志）、Redis（缓存/会话）、ClickHouse（行情/分析）	关系型 + 缓存 + 时序数据库组合
消息队列	Kafka / RabbitMQ	订单流、风控事件、行情数据的解耦
监控/日志	Prometheus + Grafana、ELK Stack（日志收集）	系统监控、业务指标可视化
参考：交易所核心架构通常包含接入层、撮合引擎、风控、账本等模块。本设计将其适配为量化交易系统，重点强化策略、风控与审计。