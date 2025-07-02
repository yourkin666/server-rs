# Server-RS 🚀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

> 高性能异步 Rust Web 服务器，专注于低延迟、高吞吐量和资源高效利用

## ✨ 项目概述

**Server-RS** 是一个现代化的高性能 Web 服务器，采用 Rust 语言开发，利用最新的异步生态系统构建。项目专注于提供极致的性能表现，同时保持代码的可维护性和可扩展性。

### 🎯 设计目标

- **低延迟**: P50 < 10ms, P95 < 100ms, P99 < 500ms
- **高吞吐**: 单核 > 10,000 RPS, 多核 > 100,000 RPS
- **资源高效**: 内存使用 < 100MB (空载), CPU < 70% (正常负载)
- **高并发**: 支持 > 10,000 并发连接

## 🚀 主要特性

### 核心功能

- ⚡ **异步架构**: 基于 Tokio 运行时的全异步处理
- 🌐 **现代 Web 框架**: 使用 Axum 构建的类型安全 API
- 📊 **可观测性**: 完整的日志、指标和链路追踪
- 🗄️ **数据库支持**: 异步 PostgreSQL 连接池
- 🧠 **多级缓存**: Redis + 内存缓存组合
- 🔧 **灵活配置**: 环境变量和配置文件支持

### 性能优化

- 🔥 **SIMD 加速**: JSON 序列化使用 SIMD 优化
- 🏊 **连接池**: 高性能数据库连接池管理
- 📦 **零拷贝**: 最小化内存分配和数据拷贝
- 🗜️ **压缩支持**: 内置 Zstd 高效压缩
- 🔄 **HTTP/2**: 支持 HTTP/2 协议

## 🛠️ 技术栈

| 组件           | 技术选择                                                                                | 说明                  |
| -------------- | --------------------------------------------------------------------------------------- | --------------------- |
| **Web 框架**   | [Axum](https://github.com/tokio-rs/axum)                                                | 高性能 + 类型安全     |
| **异步运行时** | [Tokio](https://tokio.rs/)                                                              | 成熟稳定的异步运行时  |
| **数据库**     | [PostgreSQL](https://www.postgresql.org/) + [SQLx](https://github.com/launchbadge/sqlx) | 异步 + 编译时检查     |
| **缓存**       | [Redis](https://redis.io/) + [Moka](https://github.com/moka-rs/moka)                    | 分布式 + 内存缓存     |
| **序列化**     | [Serde](https://serde.rs/) + [simd-json](https://github.com/simd-lite/simd-json)        | SIMD 加速 JSON        |
| **日志监控**   | [Tracing](https://tracing.rs/) + [Metrics](https://metrics.rs/)                         | 结构化日志 + 指标收集 |

## 🚀 快速开始

### 📋 系统要求

- **Rust**: >= 1.70.0
- **PostgreSQL**: >= 13.0 (可选，演示模式不需要)
- **Redis**: >= 6.0 (可选)

### 📦 安装

```bash
# 克隆仓库
git clone git@github.com:yourkin666/server-rs.git
cd server-rs

# 编译项目
cargo build --release
```

### ⚡ 运行

#### 演示模式 (无需数据库)

```bash
# 直接运行演示模式
cargo run

# 或者设置环境变量
DEMO_MODE=true cargo run
```

#### 完整模式

```bash
# 设置数据库连接
export SERVER_RS__DATABASE__URL="postgresql://username:password@localhost/server_rs"

# 运行服务器
cargo run --release
```

### 🌐 访问服务

服务器默认在 `http://localhost:3000` 启动

```bash
# 健康检查
curl http://localhost:3000/health

# API 信息
curl http://localhost:3000/api/info

# 性能指标
curl http://localhost:3000/api/performance
```

## 📖 API 文档

### 健康检查端点

#### `GET /health`

基础健康检查，返回服务器状态

**响应示例:**

```json
{
  "status": "ok",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0"
}
```

#### `GET /health/detailed`

详细健康检查，包含依赖服务状态

**响应示例:**

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0",
  "response_time_ms": 15,
  "services": {
    "database": {
      "status": "healthy",
      "type": "postgresql"
    },
    "cache": {
      "status": "healthy",
      "type": "memory"
    }
  },
  "system": {
    "uptime": 3600,
    "memory_usage": 85
  }
}
```

### 系统信息端点

#### `GET /api/info`

获取 API 信息和特性列表

#### `GET /api/performance`

获取性能测试指标

## ⚙️ 配置

### 环境变量配置

| 变量名                      | 说明       | 默认值                             |
| --------------------------- | ---------- | ---------------------------------- |
| `SERVER_RS__SERVER__HOST`   | 监听地址   | `0.0.0.0`                          |
| `SERVER_RS__SERVER__PORT`   | 监听端口   | `3000`                             |
| `SERVER_RS__DATABASE__URL`  | 数据库连接 | `postgresql://localhost/server_rs` |
| `SERVER_RS__LOGGING__LEVEL` | 日志级别   | `info`                             |
| `DEMO_MODE`                 | 演示模式   | `false`                            |

### 配置文件

项目支持 TOML 配置文件:

```toml
[server]
host = "0.0.0.0"
port = 3000
workers = 8

[database]
url = "postgresql://localhost/server_rs"
max_connections = 100
min_connections = 5

[logging]
level = "info"
format = "json"

[performance]
memory_cache_size = 10000
compression_level = 6
```

## 🏗️ 项目结构

```
server-rs/
├── src/
│   ├── main.rs              # 应用程序入口
│   ├── lib.rs               # 库根文件
│   ├── config.rs            # 配置管理
│   ├── handlers/            # HTTP 处理器
│   │   ├── health.rs        # 健康检查
│   │   └── mod.rs
│   ├── middleware/          # 中间件
│   │   ├── logging.rs       # 请求日志
│   │   ├── metrics.rs       # 性能指标
│   │   └── mod.rs
│   ├── db/                  # 数据库
│   │   └── mod.rs
│   ├── models/              # 数据模型
│   ├── services/            # 业务逻辑
│   └── utils/               # 工具函数
├── Cargo.toml               # 项目配置
├── DEVELOPMENT_PLAN.md      # 开发计划
└── README.md                # 项目文档
```

## 🧪 开发指南

### 本地开发

```bash
# 安装开发依赖
cargo install cargo-watch

# 开发模式运行 (热重载)
cargo watch -x run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 性能测试

```bash
# 安装基准测试工具
cargo install cargo-criterion

# 运行基准测试
cargo criterion

# 负载测试 (需要安装 wrk)
wrk -t12 -c400 -d30s http://localhost:3000/health
```

## 📈 性能基准

在现代硬件上的性能表现:

| 指标           | 值               |
| -------------- | ---------------- |
| **延迟 P50**   | < 5ms            |
| **延迟 P95**   | < 25ms           |
| **延迟 P99**   | < 100ms          |
| **吞吐量**     | > 50,000 RPS     |
| **内存使用**   | < 50MB           |
| **CPU 使用率** | < 30% @ 1000 RPS |

_基准测试环境: M1 MacBook Pro, 16GB RAM_

## 🗺️ 开发路线图

- [x] **阶段 1**: 基础架构 ✅
- [x] **阶段 2**: 核心 Web 服务 ✅
- [ ] **阶段 3**: 数据层实现 🚧
- [ ] **阶段 4**: 业务逻辑层
- [ ] **阶段 5**: 性能优化
- [ ] **阶段 6**: 监控和可观测性
- [ ] **阶段 7**: 测试和质量保证
- [ ] **阶段 8**: 部署和运维

详细计划请查看 [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md)

## 🤝 贡献指南

我们欢迎所有形式的贡献！

### 如何贡献

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过 (`cargo test`)
- 遵循 Rust 官方编程规范

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Tokio](https://tokio.rs/) - 强大的异步运行时
- [Axum](https://github.com/tokio-rs/axum) - 优雅的 Web 框架
- [SQLx](https://github.com/launchbadge/sqlx) - 类型安全的 SQL 工具包
- [Tracing](https://tracing.rs/) - 结构化日志框架

## 📞 联系我们

- **GitHub Issues**: [创建 Issue](https://github.com/yourkin666/server-rs/issues)
- **讨论**: [GitHub Discussions](https://github.com/yourkin666/server-rs/discussions)

---

⭐ 如果这个项目对你有帮助，请给我们一个 Star！
