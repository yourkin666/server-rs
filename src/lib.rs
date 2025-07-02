//! Server-RS: 高性能 Rust 服务器
//!
//! 专注于低延迟、高吞吐量和资源高效利用的异步 Web 服务器

pub mod config;
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;
pub mod utils;

pub use config::Config;

/// 应用程序错误类型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// 应用程序状态
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub cache: moka::future::Cache<String, String>,
    pub config: Config,
}

impl AppState {
    /// 创建新的应用程序状态
    pub fn new(
        db_pool: sqlx::PgPool,
        cache: moka::future::Cache<String, String>,
        config: Config,
    ) -> Self {
        Self {
            db_pool,
            cache,
            config,
        }
    }
}
