//! Server-RS 主程序入口 (演示版本)
//!
//! 高性能 Rust 服务器启动程序

use axum::{http::StatusCode, middleware, response::Json, routing::get, Router};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

use server_rs::{config::Config, middleware::request_logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 简化的日志初始化
    tracing_subscriber::fmt::init();
    info!("日志系统初始化完成");

    // 2. 加载配置
    let config = Config::load().map_err(|e| {
        eprintln!("配置加载失败: {}", e);
        e
    })?;

    // 验证配置
    if let Err(e) = config.validate() {
        eprintln!("配置验证失败: {}", e);
        std::process::exit(1);
    }

    info!("🎯 演示模式：跳过数据库和缓存初始化");

    // 3. 构建路由（演示版本）
    let app = create_demo_router();

    // 4. 启动服务器
    let addr = config.server_address();
    info!("服务器启动中，监听地址: {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    info!("🚀 服务器已启动！访问地址: http://{}", addr);
    info!("📊 健康检查: http://{}/health", addr);
    info!("📈 API 信息: http://{}/api/info", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("服务器已关闭");
    Ok(())
}

/// 创建演示路由
fn create_demo_router() -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(demo_health_check))
        .route("/api/info", get(api_info))
        .route("/api/performance", get(performance_metrics))
        // 添加中间件
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(request_logging))
}

/// 演示健康检查
async fn demo_health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "mode": "demo",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "message": "演示模式运行中，数据库功能已禁用"
    })))
}

/// API 信息端点
async fn api_info() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "name": "Server-RS",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "高性能 Rust 服务器演示",
        "mode": "demo",
        "features": {
            "async_runtime": "tokio",
            "web_framework": "axum",
            "logging": "tracing",
            "serialization": "serde_json"
        },
        "endpoints": [
            "/health",
            "/api/info",
            "/api/performance"
        ]
    })))
}

/// 性能指标端点
async fn performance_metrics() -> Result<Json<Value>, StatusCode> {
    let start_time = std::time::Instant::now();

    // 模拟一些计算
    let mut sum = 0u64;
    for i in 0..1000 {
        sum += i;
    }

    let computation_time = start_time.elapsed();

    Ok(Json(json!({
        "performance": {
            "computation_time_ms": computation_time.as_millis(),
            "memory_usage": "模拟数据",
            "uptime_seconds": get_uptime_seconds(),
            "test_computation": sum
        },
        "system": {
            "rust_version": env!("CARGO_PKG_VERSION"),
            "target": std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
            "build_time": "compile_time"
        }
    })))
}

/// 获取系统运行时间（秒）
fn get_uptime_seconds() -> u64 {
    static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let start_time = START_TIME.get_or_init(std::time::Instant::now);
    start_time.elapsed().as_secs()
}

/// 优雅关闭信号处理
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("收到关闭信号，正在优雅关闭服务器...");
}
