//! 配置管理模块
//!
//! 支持环境变量和配置文件，提供类型安全的配置访问

use serde::{Deserialize, Serialize};
use std::env;

/// 应用程序配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// 服务器配置
    pub server: ServerConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// Redis 配置
    pub redis: RedisConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    /// 监听地址
    pub host: String,
    /// 监听端口
    pub port: u16,
    /// 工作线程数
    pub workers: Option<usize>,
    /// 请求超时时间 (秒)
    pub request_timeout: u64,
    /// 启用 HTTP/2
    pub enable_http2: bool,
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// 数据库 URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
    /// 连接超时时间 (秒)
    pub connect_timeout: u64,
    /// 查询超时时间 (秒)
    pub query_timeout: u64,
}

/// Redis 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    /// Redis URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 连接超时时间 (秒)
    pub connect_timeout: u64,
}

/// 日志配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 日志格式 (json, pretty)
    pub format: String,
    /// 启用请求日志
    pub enable_request_logging: bool,
}

/// 性能配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceConfig {
    /// 内存缓存大小
    pub memory_cache_size: u64,
    /// 内存缓存 TTL (秒)
    pub memory_cache_ttl: u64,
    /// 启用压缩
    pub enable_compression: bool,
    /// 压缩级别
    pub compression_level: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
                workers: None, // 使用系统 CPU 核心数
                request_timeout: 30,
                enable_http2: true,
            },
            database: DatabaseConfig {
                url: "postgresql://localhost/server_rs".to_string(),
                max_connections: 100,
                min_connections: 5,
                connect_timeout: 10,
                query_timeout: 30,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                max_connections: 20,
                connect_timeout: 5,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                enable_request_logging: true,
            },
            performance: PerformanceConfig {
                memory_cache_size: 10_000,
                memory_cache_ttl: 300, // 5 分钟
                enable_compression: true,
                compression_level: 6,
            },
        }
    }
}

impl Config {
    /// 从环境变量和配置文件加载配置
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            // 首先加载默认配置
            .add_source(config::Config::try_from(&Self::default())?);

        // 尝试加载配置文件
        if let Ok(config_path) = env::var("CONFIG_PATH") {
            builder = builder.add_source(config::File::with_name(&config_path));
        } else {
            // 默认配置文件路径
            builder = builder
                .add_source(config::File::with_name("config/default").required(false))
                .add_source(config::File::with_name("config/local").required(false));
        }

        // 从环境变量覆盖配置
        builder = builder.add_source(config::Environment::with_prefix("SERVER_RS").separator("__"));

        builder.build()?.try_deserialize()
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        // 验证端口范围 (u16 范围是 0-65535，所以只需要检查是否为0)
        if self.server.port == 0 {
            return Err("Server port must be greater than 0".to_string());
        }

        // 验证数据库连接数
        if self.database.max_connections < self.database.min_connections {
            return Err("Database max_connections must be >= min_connections".to_string());
        }

        // 验证日志级别
        match self.logging.level.to_lowercase().as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => {}
            _ => {
                return Err(
                    "Invalid logging level. Must be one of: error, warn, info, debug, trace"
                        .to_string(),
                )
            }
        }

        // 验证日志格式
        match self.logging.format.to_lowercase().as_str() {
            "json" | "pretty" => {}
            _ => return Err("Invalid logging format. Must be 'json' or 'pretty'".to_string()),
        }

        Ok(())
    }

    /// 获取完整的服务器地址
    #[inline]
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// 是否为开发环境
    #[inline]
    pub fn is_development(&self) -> bool {
        env::var("RUST_ENV").unwrap_or_default() == "development"
    }

    /// 是否为生产环境  
    #[inline]
    pub fn is_production(&self) -> bool {
        env::var("RUST_ENV").unwrap_or_default() == "production"
    }
}
