//! 数据库模块
//!
//! 提供高性能的异步数据库连接和操作

use crate::config::DatabaseConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// 创建数据库连接池
///
/// 使用高性能配置，支持连接预热和健康检查
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    tracing::info!("创建数据库连接池，最大连接数: {}", config.max_connections);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(300)) // 5分钟空闲超时
        .max_lifetime(Duration::from_secs(1800)) // 30分钟最大生命周期
        .test_before_acquire(true) // 获取连接前测试
        .connect(&config.url)
        .await?;

    tracing::info!("数据库连接池创建成功");
    Ok(pool)
}

/// 数据库健康检查
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;

    Ok(())
}
