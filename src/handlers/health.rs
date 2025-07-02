//! 健康检查处理器
//!
//! 提供服务器和依赖服务的健康状态检查

use crate::AppState;
use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use std::time::Instant;

/// 基础健康检查
///
/// GET /health
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}

/// 详细健康检查
///
/// GET /health/detailed
pub async fn detailed_health_check(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let start_time = Instant::now();

    // 数据库健康检查
    let db_healthy = match crate::db::health_check(&app_state.db_pool).await {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("数据库健康检查失败: {}", e);
            false
        }
    };

    // 缓存健康检查 (简单检查缓存是否可用)
    let cache_healthy = {
        let test_key = "health_check_test";
        let test_value = "ok";

        // 尝试插入和获取测试值
        app_state
            .cache
            .insert(test_key.to_string(), test_value.to_string())
            .await;
        match app_state.cache.get(&test_key.to_string()).await {
            Some(value) if value == test_value => {
                app_state.cache.remove(&test_key.to_string()).await;
                true
            }
            _ => false,
        }
    };

    let response_time = start_time.elapsed().as_millis();

    let overall_status = if db_healthy && cache_healthy {
        "healthy"
    } else {
        "unhealthy"
    };

    let status_code = if overall_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let response_body = json!({
        "status": overall_status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "response_time_ms": response_time,
        "services": {
            "database": {
                "status": if db_healthy { "healthy" } else { "unhealthy" },
                "type": "postgresql"
            },
            "cache": {
                "status": if cache_healthy { "healthy" } else { "unhealthy" },
                "type": "memory"
            }
        },
        "system": {
            "uptime": get_uptime_seconds(),
            "memory_usage": get_memory_usage()
        }
    });

    Ok((status_code, Json(response_body)))
}

/// 获取系统运行时间（秒）
fn get_uptime_seconds() -> u64 {
    // 简单实现：使用进程启动时间
    // 在生产环境中可以使用更精确的系统调用
    static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start_time = START_TIME.get_or_init(Instant::now);
    start_time.elapsed().as_secs()
}

/// 获取内存使用情况（MB）
fn get_memory_usage() -> Option<u64> {
    // TODO: 实现真实的内存使用情况获取
    // 在生产环境中可以使用 procfs 或系统调用获取更精确的内存信息
    Some(0)
}
